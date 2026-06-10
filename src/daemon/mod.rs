//! Daemon beat loop for resumable log masking.
//!
//! The daemon reads from configured input files, masks PII, appends to output
//! files, and checkpoints progress so it can resume after restarts.

pub mod job;

use crate::{config::DaemonConfig, engine::Engine};
use job::run_job;

/// Process all configured jobs once (one beat).
///
/// Errors for individual jobs are logged and execution continues to the next job.
pub fn run_once(engine: &Engine, cfg: &DaemonConfig) {
    for j in &cfg.jobs {
        match run_job(j, engine, cfg) {
            Ok(stats) if stats.processed > 0 => {
                log::info!(
                    "[{}] processed {} lines in {}ms",
                    j.job_name,
                    stats.processed,
                    stats.elapsed_ms
                );
            },
            Ok(_) => {
                log::debug!("[{}] nothing new", j.job_name);
            },
            Err(e) => {
                log::error!("[{}] error: {e}", j.job_name);
            },
        }
    }
}

/// Run the beat loop until `once` is true or the process is killed.
///
/// When `once` is `true`, processes all jobs exactly once then returns.
/// Otherwise sleeps `cfg.beat_interval` ms between sweeps.
pub fn run_loop(engine: &Engine, cfg: &DaemonConfig, once: bool) {
    loop {
        run_once(engine, cfg);
        if once {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(cfg.beat_interval));
    }
}
