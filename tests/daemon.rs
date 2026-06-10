//! Integration tests for the daemon's resumable log-masking behavior.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use pii_masker::{
    config::{DaemonConfig, Job, JobFiles},
    daemon, Engine,
};

fn make_engine() -> Engine {
    Engine::builder().build().expect("engine")
}

fn daemon_cfg(tmp: &Path, max_lines: usize) -> DaemonConfig {
    DaemonConfig {
        beat_interval: 0,
        tmp_directory: tmp.to_path_buf(),
        max_lines_in_one_go: max_lines,
        run_mode: None,
        jobs: Vec::new(),
    }
}

fn write_input(dir: &Path, name: &str, lines: &[&str]) -> PathBuf {
    let path = dir.join(name);
    let mut f = fs::File::create(&path).unwrap();
    for l in lines {
        writeln!(f, "{l}").unwrap();
    }
    path
}

fn output_lines(path: &PathBuf) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

// ── Basic mask + checkpoint ───────────────────────────────────────────────────

#[test]
fn processes_up_to_max_and_checkpoints() {
    let dir = tempfile::tempdir().unwrap();
    let tmp = dir.path().to_path_buf();
    let input = write_input(
        &tmp,
        "input.log",
        &[
            "line1 ip 10.0.0.1",
            "line2 card 4111111111111111",
            "line3 clean",
            "line4 clean",
            "line5 clean",
        ],
    );
    let output = tmp.join("output.log");
    let engine = make_engine();
    let cfg = daemon_cfg(&tmp, 2);
    let job = Job {
        job_name: "test".to_string(),
        files: JobFiles { input: input.clone(), output: output.clone() },
    };

    // First beat: processes lines 1-2
    daemon::run_once(&engine, &DaemonConfig { jobs: vec![job.clone()], ..cfg.clone() });

    let lines = output_lines(&output);
    assert_eq!(lines.len(), 2, "should have 2 masked lines after first beat");
    // IP should be masked
    assert!(lines[0].contains("[IP_ADDRESS]"), "IP should be masked: {}", lines[0]);
    // Card should be masked (last 4 visible)
    assert!(!lines[1].contains("4111111111111111"), "full card should be masked: {}", lines[1]);
    assert!(lines[1].contains("1111"), "last 4 card digits should be visible: {}", lines[1]);

    // Checkpoint should exist with value 2
    let token = pii_masker::daemon::job::state_token(&input);
    let cp_path = tmp.join(format!("{token}.linenumber"));
    let cp_val: usize = fs::read_to_string(&cp_path).unwrap().trim().parse().unwrap();
    assert_eq!(cp_val, 2, "checkpoint should be 2 after processing first 2 lines");
}

// ── Resume from checkpoint ────────────────────────────────────────────────────

#[test]
fn resumes_from_checkpoint_and_appends() {
    let dir = tempfile::tempdir().unwrap();
    let tmp = dir.path().to_path_buf();
    let input =
        write_input(&tmp, "resume.log", &["line1", "line2", "line3 ip 10.0.0.2", "line4", "line5"]);
    let output = tmp.join("resume_out.log");
    let engine = make_engine();
    let cfg = daemon_cfg(&tmp, 2);

    let make_job = || Job {
        job_name: "resume".to_string(),
        files: JobFiles { input: input.clone(), output: output.clone() },
    };

    // Beat 1: lines 1-2
    daemon::run_once(&engine, &DaemonConfig { jobs: vec![make_job()], ..cfg.clone() });
    let after_beat1 = output_lines(&output);
    assert_eq!(after_beat1.len(), 2);

    // Beat 2: lines 3-4
    daemon::run_once(&engine, &DaemonConfig { jobs: vec![make_job()], ..cfg.clone() });
    let after_beat2 = output_lines(&output);
    assert_eq!(after_beat2.len(), 4, "output should have 4 lines after beat 2");
    // Line 3 should have masked IP
    assert!(after_beat2[2].contains("[IP_ADDRESS]"), "line 3 IP masked: {}", after_beat2[2]);
    // Lines 1-2 preserved unchanged
    assert_eq!(after_beat2[0], "line1");
    assert_eq!(after_beat2[1], "line2");
}

// ── Corrupt checkpoint recovery ───────────────────────────────────────────────

#[test]
fn corrupt_checkpoint_starts_from_zero() {
    let dir = tempfile::tempdir().unwrap();
    let tmp = dir.path().to_path_buf();
    let input = write_input(&tmp, "corrupt.log", &["line1", "line2", "line3"]);
    let output = tmp.join("corrupt_out.log");

    let token = pii_masker::daemon::job::state_token(&input);
    let cp_path = tmp.join(format!("{token}.linenumber"));
    // Write garbage checkpoint
    fs::write(&cp_path, "not-a-number").unwrap();

    let engine = make_engine();
    let cfg = daemon_cfg(&tmp, 5);
    let job = Job {
        job_name: "corrupt".to_string(),
        files: JobFiles { input: input.clone(), output: output.clone() },
    };
    daemon::run_once(&engine, &DaemonConfig { jobs: vec![job], ..cfg });

    let lines = output_lines(&output);
    assert_eq!(lines.len(), 3, "should process all 3 lines after corrupt checkpoint reset");
}

// ── Missing input file is non-fatal ──────────────────────────────────────────

#[test]
fn missing_input_file_logs_and_continues() {
    let dir = tempfile::tempdir().unwrap();
    let tmp = dir.path().to_path_buf();
    let engine = make_engine();
    let cfg = daemon_cfg(&tmp, 5);
    let job = Job {
        job_name: "missing".to_string(),
        files: JobFiles { input: tmp.join("does_not_exist.log"), output: tmp.join("out.log") },
    };
    // Should not panic — error is logged
    daemon::run_once(&engine, &DaemonConfig { jobs: vec![job], ..cfg });
}
