use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;

#[test]
fn documented_example_catches_every_seeded_state() {
    let output = Command::cargo_bin("sspf")
        .unwrap()
        .args([
            "check",
            "--manifest",
            "examples/preflight.toml",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["missing"], 1);
    assert_eq!(report["summary"]["extra"], 2);
    assert_eq!(report["summary"]["renamed"], 1);
    assert_eq!(report["summary"]["over_limit"], 1);
    assert_eq!(report["summary"]["blocked"], 1);
    assert_eq!(
        report["environments"][1]["destinations"][0]["likely_renamed"][0]["current"],
        "SESION_KEY"
    );
}

#[test]
fn terminal_output_marks_clean_and_blocked_destinations() {
    Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", "examples/preflight.toml"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("staging"))
        .stdout(predicate::str::contains("PASS"));
}

#[test]
fn github_mode_emits_annotations_and_blocks() {
    Command::cargo_bin("sspf")
        .unwrap()
        .args([
            "check",
            "--manifest",
            "examples/preflight.toml",
            "--format",
            "github",
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "::error title=Missing secret key::",
        ))
        .stdout(predicate::str::contains(
            "::warning title=Likely renamed key::",
        ))
        .stdout(predicate::str::contains(
            "::error title=Destination limit exceeded::",
        ));
}

#[test]
fn help_and_terminal_output_use_expected_and_destination_terms() {
    Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("expected key manifest"))
        .stdout(predicate::str::contains("desired key manifest").not());
    Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", "examples/preflight.toml"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("destination keys"))
        .stdout(predicate::str::contains("expected keys"))
        .stdout(predicate::str::contains(" current ").not())
        .stdout(predicate::str::contains(" desired").not());
}

#[test]
fn json_flag_prints_a_scriptable_report() {
    let output = Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", "examples/preflight.toml", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["summary"]["missing"], 1);
}

#[test]
fn empty_manifest_is_a_valid_noop() {
    let directory = tempfile::tempdir().unwrap();
    let manifest = directory.path().join("empty.toml");
    fs::write(&manifest, "version = 1\n").unwrap();
    Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", manifest.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("nothing to compare"));
}

#[test]
fn unsafe_export_error_never_leaks_value() {
    let directory = tempfile::tempdir().unwrap();
    let export = directory.path().join("unsafe.keys");
    let manifest = directory.path().join("preflight.toml");
    fs::write(&export, "TOKEN=super-sensitive-value\n").unwrap();
    fs::write(
        &manifest,
        "version=1\n[[environments]]\nname='prod'\ndesired=['TOKEN']\n[[environments.destinations]]\nname='ci'\nexport='unsafe.keys'\n",
    )
    .unwrap();
    Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", manifest.to_str().unwrap()])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("line 1"))
        .stderr(predicate::str::contains("super-sensitive-value").not())
        .stderr(predicate::str::contains("TOKEN").not());
}

#[test]
fn report_file_is_written_for_local_ci_artifacts() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("report.json");
    Command::cargo_bin("sspf")
        .unwrap()
        .args([
            "check",
            "--manifest",
            "examples/preflight.toml",
            "--format",
            "json",
            "--report",
            path.to_str().unwrap(),
        ])
        .assert()
        .code(1);
    let report: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(report["schema_version"], 1);
}

#[test]
fn report_cannot_overwrite_an_input_export() {
    Command::cargo_bin("sspf")
        .unwrap()
        .args([
            "check",
            "--manifest",
            "examples/preflight.toml",
            "--report",
            "examples/staging-ci.keys",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("must not overwrite"));
}

#[test]
fn warning_policy_passes_unless_strict() {
    let directory = tempfile::tempdir().unwrap();
    let export = directory.path().join("provider.keys");
    let manifest = directory.path().join("preflight.toml");
    fs::write(&export, "EXPECTED\nSTALE\n").unwrap();
    fs::write(
        &manifest,
        "version=1\n[[environments]]\nname='prod'\ndesired=['EXPECTED']\n[[environments.destinations]]\nname='ci'\nexport='provider.keys'\ndelete_policy='warn'\n",
    )
    .unwrap();
    Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", manifest.to_str().unwrap()])
        .assert()
        .success();
    Command::cargo_bin("sspf")
        .unwrap()
        .args([
            "check",
            "--manifest",
            manifest.to_str().unwrap(),
            "--strict-extra",
        ])
        .assert()
        .code(1);
}
