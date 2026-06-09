//! Individual job execution for the log-stream daemon.

use std::{
    fs,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    time::Instant,
};

use sha2::{Digest, Sha256};

use crate::{
    config::{DaemonConfig, Job},
    engine::Engine,
    error::Result,
};

/// Statistics returned after a single job beat.
pub struct JobStats {
    /// Lines processed this beat.
    pub processed: usize,
    /// Elapsed wall-clock time in milliseconds.
    pub elapsed_ms: u128,
}

/// Compute the 32-hex-char state token for an input file.
///
/// The token is `SHA-256(path + birth_secs)[..32]`, where `birth_secs` comes
/// from `metadata.created()`, falling back to `metadata.modified()`, and
/// finally to the path string alone (rotation detection disabled; a warning is
/// logged in that case).
#[must_use]
pub fn state_token(input: &Path) -> String {
    let path_str = input.display().to_string();
    let seed = fs::metadata(input)
        .ok()
        .and_then(|m| m.created().or_else(|_| m.modified()).ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .map_or_else(
            || {
                log::warn!(
                    "state_token: cannot read file times for {path_str}; rotation detection disabled"
                );
                path_str.clone()
            },
            |secs| format!("{path_str}{secs}"),
        );
    let mut h = Sha256::new();
    h.update(seed.as_bytes());
    let digest = h.finalize();
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    hex[..32].to_string()
}

fn read_checkpoint(token: &str, tmp: &Path) -> usize {
    let p = tmp.join(format!("{token}.linenumber"));
    match fs::read_to_string(&p) {
        Ok(s) => s.trim().parse::<usize>().unwrap_or_else(|_| {
            log::warn!("corrupt checkpoint {}: starting from 0", p.display());
            0
        }),
        Err(_) => 0,
    }
}

fn write_checkpoint(token: &str, tmp: &Path, line: usize) -> std::io::Result<()> {
    let p = tmp.join(format!("{token}.linenumber"));
    fs::write(p, line.to_string())
}

/// Process one daemon beat for a single job.
///
/// Reads from the checkpoint line, masks up to `cfg.max_lines_in_one_go` lines,
/// appends masked output to the output file, and writes the new checkpoint.
///
/// # Errors
/// Returns an error if the input file cannot be opened or the output file cannot be written.
pub fn run_job(job: &Job, engine: &Engine, cfg: &DaemonConfig) -> Result<JobStats> {
    let start = Instant::now();
    let input = &job.files.input;
    let output = &job.files.output;
    let tmp = &cfg.tmp_directory;

    fs::create_dir_all(tmp)?;

    let token = state_token(input);
    let start_line = read_checkpoint(&token, tmp);
    let cap = start_line + cfg.max_lines_in_one_go;

    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let in_file = fs::File::open(input)?;
    let reader = BufReader::new(in_file);
    let out_file = fs::OpenOptions::new().create(true).append(true).open(output)?;
    let mut writer = BufWriter::new(out_file);

    let mut processed = 0usize;
    let mut current_line = start_line;

    for (i, line_result) in reader.lines().enumerate() {
        let lineno = i + 1; // 1-based
        if lineno <= start_line {
            continue;
        }
        if lineno > cap {
            break;
        }
        let line = line_result?;
        let masked = engine.mask(&line);
        writer.write_all(masked.as_bytes())?;
        writer.write_all(b"\n")?;
        processed += 1;
        current_line = lineno;
    }

    writer.flush()?;

    if processed > 0 {
        write_checkpoint(&token, tmp, current_line)?;
    }

    Ok(JobStats { processed, elapsed_ms: start.elapsed().as_millis() })
}
