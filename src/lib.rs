//! Comparison engine for the `sspf` command-line preflight.
//!
//! The engine accepts only key names. Provider values and credentials are
//! deliberately outside its public data model.
//!
//! ```
//! use secret_sync_preflight::{run, Manifest, ReportStatus};
//! use std::path::Path;
//!
//! let manifest = Manifest { version: 1, environments: vec![] };
//! let report = run(&manifest, Path::new("preflight.toml"), false)?;
//! assert_eq!(report.status, ReportStatus::Pass);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub version: u8,
    #[serde(default)]
    pub environments: Vec<Environment>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    pub name: String,
    #[serde(default)]
    pub desired: Vec<String>,
    #[serde(default)]
    pub destinations: Vec<Destination>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Destination {
    pub name: String,
    pub export: PathBuf,
    pub limit: Option<usize>,
    #[serde(default)]
    pub delete_policy: DeletePolicy,
    #[serde(default)]
    pub ignore: Vec<String>,
}

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeletePolicy {
    #[default]
    Block,
    Warn,
    Allow,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub schema_version: u8,
    pub status: ReportStatus,
    pub summary: Summary,
    pub environments: Vec<EnvironmentReport>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReportStatus {
    Pass,
    Fail,
}

#[derive(Debug, Default, Serialize)]
pub struct Summary {
    pub environments: usize,
    pub destinations: usize,
    pub missing: usize,
    pub extra: usize,
    pub renamed: usize,
    pub over_limit: usize,
    pub blocked: usize,
}

#[derive(Debug, Serialize)]
pub struct EnvironmentReport {
    pub name: String,
    pub desired_count: usize,
    pub destinations: Vec<DestinationReport>,
}

#[derive(Debug, Serialize)]
pub struct DestinationReport {
    pub name: String,
    pub export: String,
    pub delete_policy: DeletePolicy,
    pub desired_count: usize,
    pub current_count: usize,
    pub missing: Vec<String>,
    pub extra: Vec<String>,
    pub likely_renamed: Vec<Rename>,
    pub capacity: Option<Capacity>,
    pub blocked: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Rename {
    pub current: String,
    pub desired: String,
}

#[derive(Debug, Serialize)]
pub struct Capacity {
    pub limit: usize,
    pub desired: usize,
    pub current: usize,
    pub at_limit: bool,
    pub over_limit: bool,
}

#[derive(Debug)]
pub struct PreflightError(String);

impl fmt::Display for PreflightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PreflightError {}

fn error(message: impl Into<String>) -> PreflightError {
    PreflightError(message.into())
}

/// Load and validate a version 1 TOML manifest.
pub fn load_manifest(path: &Path) -> Result<Manifest, PreflightError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| error(format!("cannot read manifest {}: {e}", path.display())))?;
    let manifest: Manifest = toml::from_str(&source).map_err(|_| {
        error(format!(
            "invalid manifest {}: TOML syntax or schema error (details suppressed to prevent metadata leakage)",
            path.display()
        ))
    })?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

fn validate_manifest(manifest: &Manifest) -> Result<(), PreflightError> {
    if manifest.version != 1 {
        return Err(error(format!(
            "unsupported manifest version {}; expected 1",
            manifest.version
        )));
    }
    let mut environment_names = HashSet::new();
    for environment in &manifest.environments {
        validate_label("environment", &environment.name)?;
        if !environment_names.insert(&environment.name) {
            return Err(error(format!(
                "duplicate environment name {:?}",
                environment.name
            )));
        }
        validate_key_list(
            &environment.desired,
            &format!("environment {:?} desired", environment.name),
        )?;
        let mut destination_names = HashSet::new();
        for destination in &environment.destinations {
            validate_label("destination", &destination.name)?;
            if !destination_names.insert(&destination.name) {
                return Err(error(format!(
                    "duplicate destination {:?} in environment {:?}",
                    destination.name, environment.name
                )));
            }
            if destination.export.as_os_str().is_empty() {
                return Err(error(format!(
                    "destination {:?} has an empty export path",
                    destination.name
                )));
            }
            if destination.limit == Some(0) {
                return Err(error(format!(
                    "destination {:?} limit must be greater than zero",
                    destination.name
                )));
            }
            validate_key_list(
                &destination.ignore,
                &format!("destination {:?} ignore", destination.name),
            )?;
        }
    }
    Ok(())
}

fn validate_label(kind: &str, value: &str) -> Result<(), PreflightError> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(error(format!(
            "{kind} name must be non-empty and contain no control characters"
        )));
    }
    Ok(())
}

fn validate_key_list(keys: &[String], context: &str) -> Result<(), PreflightError> {
    let mut seen = HashSet::new();
    for key in keys {
        if !is_valid_key(key) {
            return Err(error(format!("{context} contains an invalid key name")));
        }
        if !seen.insert(key) {
            return Err(error(format!("{context} contains a duplicate key name")));
        }
    }
    Ok(())
}

fn is_valid_key(key: &str) -> bool {
    !key.is_empty()
        && key.len() <= 255
        && key.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'/' | b':')
        })
}

/// Read a key-only export. Values are rejected without being retained or echoed.
pub fn read_key_export(path: &Path) -> Result<BTreeSet<String>, PreflightError> {
    let file = File::open(path)
        .map_err(|e| error(format!("cannot read key export {}: {e}", path.display())))?;
    let mut keys = BTreeSet::new();
    for (index, line_result) in BufReader::new(file).lines().enumerate() {
        let mut line = line_result.map_err(|e| {
            error(format!(
                "cannot read key export {} at line {}: {e}",
                path.display(),
                index + 1
            ))
        })?;
        if index == 0 {
            line = line.trim_start_matches('\u{feff}').to_owned();
        }
        let candidate = line.trim();
        if candidate.is_empty() || candidate.starts_with('#') {
            continue;
        }
        if candidate.contains('=')
            || candidate.split_whitespace().count() != 1
            || !is_valid_key(candidate)
        {
            return Err(error(format!(
                "key export {} line {} is not a key-only record; remove values and structured data",
                path.display(),
                index + 1
            )));
        }
        if !keys.insert(candidate.to_owned()) {
            return Err(error(format!(
                "key export {} contains a duplicate at line {}",
                path.display(),
                index + 1
            )));
        }
    }
    Ok(keys)
}

/// Run every environment/destination comparison in a validated manifest.
pub fn run(
    manifest: &Manifest,
    manifest_path: &Path,
    strict_extra: bool,
) -> Result<Report, PreflightError> {
    let base = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let mut summary = Summary {
        environments: manifest.environments.len(),
        ..Summary::default()
    };
    let mut environment_reports = Vec::new();

    for environment in &manifest.environments {
        let desired_all: BTreeSet<_> = environment.desired.iter().cloned().collect();
        let mut destination_reports = Vec::new();
        for destination in &environment.destinations {
            let export_path = if destination.export.is_absolute() {
                destination.export.clone()
            } else {
                base.join(&destination.export)
            };
            let ignore: BTreeSet<_> = destination.ignore.iter().cloned().collect();
            let desired: BTreeSet<_> = desired_all.difference(&ignore).cloned().collect();
            let current: BTreeSet<_> = read_key_export(&export_path)?
                .difference(&ignore)
                .cloned()
                .collect();
            let missing: Vec<_> = desired.difference(&current).cloned().collect();
            let extra: Vec<_> = current.difference(&desired).cloned().collect();
            let likely_renamed = likely_renames(&missing, &extra);
            let capacity = destination.limit.map(|limit| Capacity {
                limit,
                desired: desired.len(),
                current: current.len(),
                at_limit: desired.len() == limit || current.len() == limit,
                over_limit: desired.len() > limit || current.len() > limit,
            });
            let mut reasons = Vec::new();
            if !missing.is_empty() {
                reasons.push(format!("{} required key(s) missing", missing.len()));
            }
            if capacity.as_ref().is_some_and(|value| value.over_limit) {
                reasons.push("provider key limit exceeded".to_owned());
            }
            if !extra.is_empty()
                && (strict_extra || matches!(destination.delete_policy, DeletePolicy::Block))
            {
                reasons.push(match destination.delete_policy {
                    DeletePolicy::Block => format!(
                        "{} extra key(s) would require a blocked deletion",
                        extra.len()
                    ),
                    _ => format!("{} extra key(s) rejected by strict mode", extra.len()),
                });
            }
            let blocked = !reasons.is_empty();
            summary.destinations += 1;
            summary.missing += missing.len();
            summary.extra += extra.len();
            summary.renamed += likely_renamed.len();
            summary.over_limit +=
                usize::from(capacity.as_ref().is_some_and(|value| value.over_limit));
            summary.blocked += usize::from(blocked);
            destination_reports.push(DestinationReport {
                name: destination.name.clone(),
                export: destination.export.display().to_string(),
                delete_policy: destination.delete_policy,
                desired_count: desired.len(),
                current_count: current.len(),
                missing,
                extra,
                likely_renamed,
                capacity,
                blocked,
                reasons,
            });
        }
        environment_reports.push(EnvironmentReport {
            name: environment.name.clone(),
            desired_count: desired_all.len(),
            destinations: destination_reports,
        });
    }

    Ok(Report {
        schema_version: 1,
        status: if summary.blocked == 0 {
            ReportStatus::Pass
        } else {
            ReportStatus::Fail
        },
        summary,
        environments: environment_reports,
    })
}

fn likely_renames(missing: &[String], extra: &[String]) -> Vec<Rename> {
    let mut candidates = Vec::new();
    for (missing_index, wanted) in missing.iter().enumerate() {
        for (extra_index, current) in extra.iter().enumerate() {
            let distance = levenshtein(&wanted.to_ascii_lowercase(), &current.to_ascii_lowercase());
            if wanted.len().max(current.len()) >= 5 && distance > 0 && distance <= 2 {
                candidates.push((distance, missing_index, extra_index));
            }
        }
    }
    candidates.sort();
    let mut used_missing = HashSet::new();
    let mut used_extra = HashSet::new();
    let mut renames = Vec::new();
    for (_, missing_index, extra_index) in candidates {
        if used_missing.insert(missing_index) && used_extra.insert(extra_index) {
            renames.push(Rename {
                current: extra[extra_index].clone(),
                desired: missing[missing_index].clone(),
            });
        }
    }
    renames.sort_by(|a, b| a.desired.cmp(&b.desired));
    renames
}

fn levenshtein(a: &str, b: &str) -> usize {
    let mut previous: Vec<usize> = (0..=b.chars().count()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut current = vec![i + 1];
        for (j, cb) in b.chars().enumerate() {
            current.push(
                (previous[j + 1] + 1)
                    .min(current[j] + 1)
                    .min(previous[j] + usize::from(ca != cb)),
            );
        }
        previous = current;
    }
    previous[b.chars().count()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_likely_rename() {
        let result = likely_renames(&["SESSION_KEY".into()], &["SESION_KEY".into()]);
        assert_eq!(
            result,
            vec![Rename {
                current: "SESION_KEY".into(),
                desired: "SESSION_KEY".into()
            }]
        );
    }

    #[test]
    fn rejects_assignment_without_echoing_it() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("unsafe.keys");
        std::fs::write(&path, "SAFE_KEY\nPASSWORD=do-not-print\n").unwrap();
        let message = read_key_export(&path).unwrap_err().to_string();
        assert!(message.contains("line 2"));
        assert!(!message.contains("do-not-print"));
        assert!(!message.contains("PASSWORD"));
    }

    #[test]
    fn accepts_empty_export() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("empty.keys");
        std::fs::write(&path, "# key-only export\n\n").unwrap();
        assert!(read_key_export(&path).unwrap().is_empty());
    }
}
