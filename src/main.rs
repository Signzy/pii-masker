//! Binary entry point — requires the `cli` feature.

#[cfg(not(feature = "cli"))]
compile_error!("The pii-masker binary requires the `cli` feature (enabled by default).");

#[cfg(feature = "cli")]
fn main() -> anyhow::Result<()> {
    use std::{
        fs,
        io::{self, BufRead, BufReader, Write},
    };

    use clap::Parser;
    use pii_masker::{
        cli::{Cli, Cmd},
        config, daemon,
        engine::EngineBuilder,
        MaskPolicy,
    };

    let cli = Cli::parse();

    // Initialise env_logger. RUST_LOG overrides -v.
    let level = if cli.verbose > 0 { "debug" } else { "info" };
    env_logger::Builder::new()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| level.to_string()))
        .init();

    let cfg = config::load_or_default(cli.config.as_deref());

    // Build engine, applying CLI country selection over config's selection.
    let build_engine = |country: Option<&str>, all: bool| -> anyhow::Result<pii_masker::Engine> {
        if all {
            EngineBuilder::default().all_countries().build().map_err(|e| anyhow::anyhow!("{e}"))
        } else if let Some(codes) = country {
            let list: Vec<String> =
                codes.split(',').map(|s| s.trim().to_ascii_uppercase()).collect();
            EngineBuilder::default().countries(list).build().map_err(|e| anyhow::anyhow!("{e}"))
        } else {
            cfg.engine.build_engine().map_err(|e| anyhow::anyhow!("{e}"))
        }
    };

    match cli.command {
        Cmd::Mask { input, output, country, all_countries } => {
            let engine = build_engine(country.as_deref(), all_countries)?;
            let reader: Box<dyn BufRead> = match &input {
                Some(p) => Box::new(BufReader::new(fs::File::open(p)?)),
                None => Box::new(BufReader::new(io::stdin())),
            };
            let stdout = io::stdout();
            let mut writer: Box<dyn Write> = match &output {
                Some(p) => Box::new(io::BufWriter::new(fs::File::create(p)?)),
                None => Box::new(io::BufWriter::new(stdout.lock())),
            };
            for line in reader.lines() {
                let line = line?;
                let masked = engine.mask(&line);
                writeln!(writer, "{masked}")?;
            }
        },

        Cmd::Scan { input, json, country, all_countries } => {
            let engine = build_engine(country.as_deref(), all_countries)?;
            let reader: Box<dyn BufRead> = match &input {
                Some(p) => Box::new(BufReader::new(fs::File::open(p)?)),
                None => Box::new(BufReader::new(io::stdin())),
            };
            let mut all_findings: Vec<serde_json::Value> = Vec::new();
            let mut byte_offset = 0usize;
            for line in reader.lines() {
                let line = line?;
                for f in engine.analyze(&line) {
                    if json {
                        all_findings.push(serde_json::json!({
                            "entity": f.entity,
                            "country": f.country,
                            "start": byte_offset + f.start,
                            "end": byte_offset + f.end,
                            "score": f.score,
                        }));
                    } else {
                        println!(
                            "{:<25} {:>6}:{:<6} score={:.2}",
                            f.entity,
                            byte_offset + f.start,
                            byte_offset + f.end,
                            f.score
                        );
                    }
                }
                byte_offset += line.len() + 1; // +1 for '\n'
            }
            if json {
                println!("{}", serde_json::to_string_pretty(&all_findings)?);
            }
        },

        Cmd::Run { once } => {
            let engine = cfg.engine.build_engine().map_err(|e| anyhow::anyhow!("{e}"))?;
            daemon::run_loop(&engine, &cfg.daemon, once);
        },

        Cmd::List { country, all_countries, json } => {
            let engine = build_engine(country.as_deref(), all_countries)?;
            if json {
                let arr: Vec<serde_json::Value> = engine
                    .recognizers()
                    .iter()
                    .map(|r| {
                        serde_json::json!({
                            "entity": r.entity,
                            "country": r.country,
                            "has_checksum": r.validator.has_checksum(),
                            "score": r.score,
                            "mask_kind": match &r.mask {
                                MaskPolicy::Partial { .. } => "partial",
                                MaskPolicy::Redact { .. } => "redact",
                                MaskPolicy::Hash { .. } => "hash",
                            },
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&arr)?);
            } else {
                println!("{:<25} {:<5} {:<12} {:>5}  MASK", "ENTITY", "CC", "CHECKSUM", "SCORE");
                for r in engine.recognizers() {
                    let mask_kind = match &r.mask {
                        MaskPolicy::Partial { .. } => "partial",
                        MaskPolicy::Redact { .. } => "redact",
                        MaskPolicy::Hash { .. } => "hash",
                    };
                    println!(
                        "{:<25} {:<5} {:<12} {:>5.2}  {}",
                        r.entity,
                        r.country.as_deref().unwrap_or("-"),
                        if r.validator.has_checksum() { "yes" } else { "no" },
                        r.score,
                        mask_kind,
                    );
                }
            }
        },
    }

    Ok(())
}
