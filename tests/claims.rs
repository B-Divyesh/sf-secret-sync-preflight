use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;

#[test]
fn claim_demo_command_uses_bundled_key_only_files() { // @claim:demo-command
    Command::cargo_bin("sspf")
        .unwrap()
        .arg("demo")
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Demo files:"))
        .stdout(predicate::str::contains("Sample data contains key names only."))
        .stdout(predicate::str::contains("1 missing, 2 extra, 1 likely renamed"));
}

#[test]
fn claim_check_finds_the_advertised_drift_states() { // @claim:drift-report
    let output = Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", "examples/preflight.toml", "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["missing"], 1);
    assert_eq!(report["summary"]["extra"], 2);
    assert_eq!(report["summary"]["renamed"], 1);
    assert_eq!(report["summary"]["over_limit"], 1);
}

#[test]
fn claim_values_are_rejected_without_echoing_them() { // @claim:no-secret-values
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("keys"), "TOKEN=claim-sentinel-never-print\n").unwrap();
    fs::write(directory.path().join("preflight.toml"), "version=1\n[[environments]]\nname='demo'\ndesired=['TOKEN']\n[[environments.destinations]]\nname='ci'\nexport='keys'\n").unwrap();
    Command::cargo_bin("sspf").unwrap()
        .args(["check", "--manifest", directory.path().join("preflight.toml").to_str().unwrap()])
        .assert().code(2)
        .stderr(predicate::str::contains("line 1"))
        .stderr(predicate::str::contains("claim-sentinel-never-print").not());
}

#[test]
fn claim_exit_codes_describe_pass_drift_and_input_error() { // @claim:ci-exit-codes
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("keys"), "EXPECTED\n").unwrap();
    fs::write(directory.path().join("pass.toml"), "version=1\n[[environments]]\nname='demo'\ndesired=['EXPECTED']\n[[environments.destinations]]\nname='ci'\nexport='keys'\n").unwrap();
    let binary = Command::cargo_bin("sspf").unwrap();
    Command::new(binary.get_program()).args(["check", "--manifest", directory.path().join("pass.toml").to_str().unwrap()]).assert().code(0);
    Command::cargo_bin("sspf").unwrap().args(["check", "--manifest", "examples/preflight.toml"]).assert().code(1);
    Command::cargo_bin("sspf").unwrap().args(["check", "--manifest", "not-a-file.toml"]).assert().code(2);
}

#[test]
fn claim_json_github_and_local_report_are_observable() { // @claim:report-formats
    let directory = tempfile::tempdir().unwrap();
    let report = directory.path().join("report.json");
    Command::cargo_bin("sspf").unwrap()
        .args(["check", "--manifest", "examples/preflight.toml", "--format", "github", "--report", report.to_str().unwrap()])
        .assert().code(1)
        .stdout(predicate::str::contains("::error title=Missing secret key::"));
    assert!(report.exists());
    assert_eq!(serde_json::from_str::<Value>(&fs::read_to_string(report).unwrap()).unwrap()["schema_version"], 1);
}
