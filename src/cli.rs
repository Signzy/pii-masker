//! Command-line interface definition (requires feature `cli`).

use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Banking-grade PII detection and masking.
#[derive(Debug, Parser)]
#[command(name = "pii-masker", version, author, about)]
pub struct Cli {
    /// Path to JSON config file. Falls back to `pii-masker.json` if present.
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Increase log verbosity (use `-v` for DEBUG level).
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Subcommand to run.
    #[command(subcommand)]
    pub command: Cmd,
}

/// Available subcommands.
#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Mask PII in an input file or stdin, writing to an output file or stdout.
    Mask {
        /// Input file path (defaults to stdin).
        #[arg(long, short = 'i')]
        input: Option<PathBuf>,
        /// Output file path (defaults to stdout).
        #[arg(long, short = 'o')]
        output: Option<PathBuf>,
        /// Comma-separated ISO-3166-1 alpha-2 country codes to activate (e.g. `US,IN`).
        #[arg(long)]
        country: Option<String>,
        /// Activate all country packs.
        #[arg(long)]
        all_countries: bool,
    },
    /// Scan for PII findings and report them (does not modify input).
    Scan {
        /// Input file path (defaults to stdin).
        #[arg(long, short = 'i')]
        input: Option<PathBuf>,
        /// Emit findings as a JSON array.
        #[arg(long)]
        json: bool,
        /// Comma-separated country codes to activate.
        #[arg(long)]
        country: Option<String>,
        /// Activate all country packs.
        #[arg(long)]
        all_countries: bool,
    },
    /// Run the log-stream daemon using configured jobs.
    Run {
        /// Process all jobs exactly once then exit (do not loop).
        #[arg(long)]
        once: bool,
    },
    /// List available recognizers in the active catalog.
    List {
        /// Comma-separated country codes to activate.
        #[arg(long)]
        country: Option<String>,
        /// Activate all country packs.
        #[arg(long)]
        all_countries: bool,
        /// Emit recognizer list as a JSON array.
        #[arg(long)]
        json: bool,
    },
}
