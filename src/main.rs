use clap::{Parser, Subcommand, ValueEnum};
use secret_sync_preflight::{load_manifest, run, Report, ReportStatus};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "sspf",
    version,
    about = "Check secret key parity before deployment",
    long_about = "Read-only preflight for secret key names across CI and hosting destinations. Values are rejected, never printed, and never stored."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compare a desired key manifest with destination key exports
    Check {
        /// Path to the version 1 TOML manifest
        #[arg(short, long, default_value = "preflight.toml")]
        manifest: PathBuf,
        /// Output format written to stdout
        #[arg(long, value_enum, default_value = "terminal")]
        format: OutputFormat,
        /// Also write the full JSON report to this local path
        #[arg(long)]
        report: Option<PathBuf>,
        /// Fail on extra keys even when the destination policy allows them
        #[arg(long)]
        strict_extra: bool,
    },
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
            report,
            strict_extra,
        } => {
            if report.as_ref().is_some_and(|path| path == &manifest) {
                return Err("report path must not overwrite the manifest".to_owned());
            }
            let manifest_data = load_manifest(&manifest).map_err(|e| e.to_string())?;
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
    }
}

fn print_terminal(report: &Report) {
    if report.environments.is_empty() {
        println!("PASS  No environments configured; nothing to compare.");
        return;
    }
    for environment in &report.environments {
        println!(
            "ENV   {} ({} desired)",
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
                "  {state} {} — {} current / {} desired",
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
                    "    LIMIT   {state}: desired {}, current {}, maximum {}",
                    capacity.desired, capacity.current, capacity.limit
                );
            }
            for reason in &destination.reasons {
                println!("    WHY     {reason}");
            }
        }
    }
    println!(
        "{}  {} destination(s), {} missing, {} extra, {} likely renamed, {} over limit",
        if report.status == ReportStatus::Pass {
            "PASS"
        } else {
            "FAIL"
        },
        report.summary.destinations,
        report.summary.missing,
        report.summary.extra,
        report.summary.renamed,
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
                    println!("::error title=Provider limit exceeded::{} / {}: desired {}%2C current {}%2C maximum {}", escape(&environment.name), escape(&destination.name), capacity.desired, capacity.current, capacity.limit);
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
