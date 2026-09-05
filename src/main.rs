use clap::{Parser, Subcommand, ValueEnum};
use secret_sync_preflight::{load_manifest, run, Report, ReportStatus};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "sspf",
    version,
    about = "Check secret key drift before deployment",
    long_about = "Compare expected secret key names with destination key exports. Values are rejected and never printed."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compare an expected key manifest with destination key exports
    Check {
        /// Path to the version 1 TOML manifest
        #[arg(short, long, default_value = "preflight.toml")]
        manifest: PathBuf,
        /// Output format written to stdout
        #[arg(long, value_enum, default_value = "terminal")]
        format: OutputFormat,
        /// Print the JSON report to stdout
        #[arg(long, conflicts_with = "format")]
        json: bool,
        /// Also write the full JSON report to this local path
        #[arg(long)]
        report: Option<PathBuf>,
        /// Fail on extra keys even when the destination policy allows them
        #[arg(long)]
        strict_extra: bool,
    },
    /// Run the bundled sample in a new temporary directory
    Demo,
}

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Terminal,
    Json,
    Github,
}

fn main() {
    let code = match execute(Cli::parse()) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("sspf: {message}");
            2
        }
    };
    std::process::exit(code);
}

fn execute(cli: Cli) -> Result<i32, String> {
    match cli.command {
        Command::Check {
            manifest,
            format,
            json,
            report,
            strict_extra,
        } => {
            let format = if json { OutputFormat::Json } else { format };
            let manifest_data = load_manifest(&manifest).map_err(|e| e.to_string())?;
            if let Some(report_path) = &report {
                let mut inputs = vec![manifest.clone()];
                let base = manifest
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."));
                for environment in &manifest_data.environments {
                    for destination in &environment.destinations {
                        inputs.push(if destination.export.is_absolute() {
                            destination.export.clone()
                        } else {
                            base.join(&destination.export)
                        });
                    }
                }
                if inputs
                    .iter()
                    .any(|input| same_existing_file(report_path, input))
                {
                    return Err(
                        "report path must not overwrite the manifest or a key export".to_owned(),
                    );
                }
            }
            let result = run(&manifest_data, &manifest, strict_extra).map_err(|e| e.to_string())?;
            let json = serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?;
            if let Some(path) = report {
                std::fs::write(&path, format!("{json}\n"))
                    .map_err(|e| format!("cannot write report {}: {e}", path.display()))?;
            }
            match format {
                OutputFormat::Terminal => print_terminal(&result),
                OutputFormat::Json => println!("{json}"),
                OutputFormat::Github => print_github(&result),
            }
            Ok(if result.status == ReportStatus::Pass {
                0
            } else {
                1
            })
        }
        Command::Demo => run_demo(),
    }
}

/// Materialize the safe, key-name-only fixture so a first run never needs a
/// repository, credentials, or a provider connection. The directory is left
/// behind deliberately: the user can inspect the exact files that were read.
fn run_demo() -> Result<i32, String> {
    let nonce = format!(
        "sspf-demo-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos()
    );
    let directory = std::env::temp_dir().join(nonce);
    std::fs::create_dir_all(&directory)
        .map_err(|e| format!("cannot create demo directory {}: {e}", directory.display()))?;
    let manifest = directory.join("preflight.toml");
    std::fs::write(&manifest, include_str!("../examples/preflight.toml"))
        .map_err(|e| format!("cannot write demo manifest: {e}"))?;
    std::fs::write(
        directory.join("staging-ci.keys"),
        include_str!("../examples/staging-ci.keys"),
    )
    .map_err(|e| format!("cannot write demo export: {e}"))?;
    std::fs::write(
        directory.join("production-hosting.keys"),
        include_str!("../examples/production-hosting.keys"),
    )
    .map_err(|e| format!("cannot write demo export: {e}"))?;
    let manifest_data = load_manifest(&manifest).map_err(|e| e.to_string())?;
    let report = run(&manifest_data, &manifest, false).map_err(|e| e.to_string())?;
    println!("Demo files: {}", directory.display());
    println!("Sample data contains key names only. No provider login or network is used.");
    print_terminal(&report);
    Ok(if report.status == ReportStatus::Pass {
        0
    } else {
        1
    })
}

fn same_existing_file(left: &std::path::Path, right: &std::path::Path) -> bool {
    match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => left == right,
    }
}

fn print_terminal(report: &Report) {
    if report.environments.is_empty() {
        println!("PASS  No environments configured; nothing to compare.");
        return;
    }
    for environment in &report.environments {
        println!(
            "ENV   {} ({} expected keys)",
            environment.name, environment.desired_count
        );
        if environment.destinations.is_empty() {
            println!("  PASS  No destinations configured.");
        }
        for destination in &environment.destinations {
            let state = if destination.blocked {
                "BLOCK"
            } else {
                "PASS "
            };
            println!(
                "  {state} {} — {} destination keys / {} expected keys",
                destination.name, destination.current_count, destination.desired_count
            );
            for rename in &destination.likely_renamed {
                println!("    RENAME? {} -> {}", rename.current, rename.desired);
            }
            for key in &destination.missing {
                println!("    MISSING {key}");
            }
            for key in &destination.extra {
                println!("    EXTRA   {key}");
            }
            if let Some(capacity) = &destination.capacity {
                let state = if capacity.over_limit {
                    "OVER"
                } else if capacity.at_limit {
                    "AT"
                } else {
                    "OK"
                };
                println!(
                    "    LIMIT   {state}: expected {}, destination {}, maximum {}",
                    capacity.desired, capacity.current, capacity.limit
                );
            }
            for reason in &destination.reasons {
                println!("    WHY     {reason}");
            }
        }
    }
    println!(
        "{}  {} {}, {} missing, {} extra, {} likely {}, {} over limit",
        if report.status == ReportStatus::Pass {
            "PASS"
        } else {
            "FAIL"
        },
        report.summary.destinations,
        if report.summary.destinations == 1 {
            "destination"
        } else {
            "destinations"
        },
        report.summary.missing,
        report.summary.extra,
        report.summary.renamed,
        if report.summary.renamed == 1 {
            "rename"
        } else {
            "renames"
        },
        report.summary.over_limit
    );
}

fn print_github(report: &Report) {
    if report.environments.is_empty() {
        println!(
            "::notice title=Secret Sync Preflight::No environments configured; nothing to compare"
        );
    }
    for environment in &report.environments {
        for destination in &environment.destinations {
            for rename in &destination.likely_renamed {
                println!(
                    "::warning title=Likely renamed key::{} / {}: {} may be {}",
                    escape(&environment.name),
                    escape(&destination.name),
                    escape(&rename.current),
                    escape(&rename.desired)
                );
            }
            for key in &destination.missing {
                println!(
                    "::error title=Missing secret key::{} / {}: {}",
                    escape(&environment.name),
                    escape(&destination.name),
                    escape(key)
                );
            }
            for key in &destination.extra {
                let level = if destination.blocked {
                    "error"
                } else {
                    "warning"
                };
                println!(
                    "::{level} title=Extra secret key::{} / {}: {}",
                    escape(&environment.name),
                    escape(&destination.name),
                    escape(key)
                );
            }
            if let Some(capacity) = &destination.capacity {
                if capacity.over_limit {
                    println!("::error title=Destination limit exceeded::{} / {}: expected {}%2C destination {}%2C maximum {}", escape(&environment.name), escape(&destination.name), capacity.desired, capacity.current, capacity.limit);
                }
            }
        }
    }
    println!(
        "::{} title=Secret Sync Preflight::{} destination(s) checked%2C {} blocked",
        if report.status == ReportStatus::Pass {
            "notice"
        } else {
            "error"
        },
        report.summary.destinations,
        report.summary.blocked
    );
}

fn escape(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}
