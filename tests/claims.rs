use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn manifest_for(export: &str, expected: &[&str], policy: &str) -> String {
    let expected = expected
        .iter()
        .map(|key| format!("'{key}'"))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "version=1\n[[environments]]\nname='demo'\ndesired=[{expected}]\n[[environments.destinations]]\nname='ci'\nexport='{export}'\ndelete_policy='{policy}'\n"
    )
}

fn directory_entries(path: &Path) -> BTreeSet<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect()
}

fn normalize_demo_path(output: &str) -> String {
    let first_line = output.lines().next().expect("demo path line");
    let path = first_line
        .strip_prefix("Demo files: ")
        .expect("demo path prefix");
    output.replacen(path, "/tmp/sspf-demo-<id>", 1)
}

#[test]
fn claim_demo_command_uses_bundled_key_only_files() {
    // @claim:demo-command
    let working = tempfile::tempdir().unwrap();
    let before = directory_entries(working.path());
    let output = Command::cargo_bin("sspf")
        .unwrap()
        .current_dir(working.path())
        .arg("demo")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    assert_eq!(directory_entries(working.path()), before);

    let stdout = String::from_utf8(output.stdout).unwrap();
    let demo_path = PathBuf::from(
        stdout
            .lines()
            .next()
            .unwrap()
            .strip_prefix("Demo files: ")
            .unwrap(),
    );
    assert!(demo_path.is_dir());
    assert!(demo_path.starts_with(std::env::temp_dir()));
    assert!(demo_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .starts_with("sspf-demo-"));

    for name in [
        "preflight.toml",
        "staging-ci.keys",
        "production-hosting.keys",
    ] {
        assert_eq!(
            fs::read(demo_path.join(name)).unwrap(),
            fs::read(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("examples")
                    .join(name)
            )
            .unwrap()
        );
    }
    assert_eq!(
        normalize_demo_path(&stdout),
        include_str!("../site/public/cli-demo-output.txt")
    );
    fs::remove_dir_all(demo_path).unwrap();
}

#[test]
fn claim_check_finds_the_advertised_drift_states() {
    // @claim:drift-report
    let inputs = [
        "examples/preflight.toml",
        "examples/staging-ci.keys",
        "examples/production-hosting.keys",
    ];
    let before: Vec<Vec<u8>> = inputs.iter().map(|path| fs::read(path).unwrap()).collect();
    let output = Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", "examples/preflight.toml", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["summary"]["missing"], 1);
    assert_eq!(report["summary"]["extra"], 2);
    assert_eq!(report["summary"]["renamed"], 1);
    assert_eq!(report["summary"]["over_limit"], 1);
    assert_eq!(report["summary"]["blocked"], 1);
    assert_eq!(report["environments"].as_array().unwrap().len(), 2);
    assert_eq!(
        report["environments"][1]["destinations"][0]["likely_renamed"][0]["current"],
        "SESION_KEY"
    );
    assert_eq!(
        report["environments"][1]["destinations"][0]["likely_renamed"][0]["desired"],
        "SESSION_KEY"
    );
    for (path, contents) in inputs.iter().zip(before) {
        assert_eq!(fs::read(path).unwrap(), contents);
    }
}

#[test]
fn claim_values_are_rejected_without_echoing_them() {
    // @claim:no-secret-values
    let directory = tempfile::tempdir().unwrap();
    let export = directory.path().join("keys");
    let manifest = directory.path().join("preflight.toml");
    fs::write(&manifest, manifest_for("keys", &["TOKEN"], "block")).unwrap();

    let cases = [
        ("TOKEN=claim-sentinel-assignment", "terminal"),
        (r#"{"TOKEN":"claim-sentinel-json"}"#, "json"),
        ("TOKEN claim-sentinel-space", "github"),
    ];
    for (index, (unsafe_line, format)) in cases.iter().enumerate() {
        fs::write(&export, format!("{unsafe_line}\n")).unwrap();
        let report = directory.path().join(format!("report-{index}.json"));
        let output = Command::cargo_bin("sspf")
            .unwrap()
            .args([
                "check",
                "--manifest",
                manifest.to_str().unwrap(),
                "--format",
                format,
                "--report",
                report.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(combined.contains("line 1"));
        assert!(!combined.contains(unsafe_line));
        assert!(!combined.contains("claim-sentinel"));
        assert!(output.stdout.is_empty());
        assert!(!report.exists());
    }
}

#[test]
fn claim_key_file_rules_accept_and_reject_the_documented_inputs() {
    // @claim:key-file-rules
    let directory = tempfile::tempdir().unwrap();
    let export = directory.path().join("keys");
    let manifest = directory.path().join("preflight.toml");
    let allowed = "AZaz09_-./:";
    fs::write(&manifest, manifest_for("keys", &[allowed], "block")).unwrap();
    fs::write(&export, format!("# destination export\n\n{allowed}\n")).unwrap();
    let with_comments = Command::cargo_bin("sspf")
        .unwrap()
        .args([
            "check",
            "--manifest",
            manifest.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(with_comments.status.code(), Some(0));
    fs::write(&export, format!("{allowed}\n")).unwrap();
    let stripped = Command::cargo_bin("sspf")
        .unwrap()
        .args([
            "check",
            "--manifest",
            manifest.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(stripped.status.code(), Some(0));
    assert_eq!(with_comments.stdout, stripped.stdout);

    for rejected in ["KEY@NAME", "KEY+NAME", "KEY\\NAME", "KÉY", "TWO KEYS"] {
        fs::write(&export, format!("{rejected}\n")).unwrap();
        Command::cargo_bin("sspf")
            .unwrap()
            .args(["check", "--manifest", manifest.to_str().unwrap()])
            .assert()
            .code(2)
            .stderr(predicate::str::contains("line 1 must contain one key name"));
    }
}

#[test]
fn claim_delete_policy_and_strict_mode_cover_every_policy() {
    // @claim:delete-policy
    let directory = tempfile::tempdir().unwrap();
    let export = directory.path().join("keys");
    fs::write(&export, "EXPECTED\nEXTRA\n").unwrap();

    for (policy, normal_code) in [("block", 1), ("warn", 0), ("allow", 0)] {
        let manifest = directory.path().join(format!("{policy}.toml"));
        fs::write(&manifest, manifest_for("keys", &["EXPECTED"], policy)).unwrap();
        Command::cargo_bin("sspf")
            .unwrap()
            .args(["check", "--manifest", manifest.to_str().unwrap()])
            .assert()
            .code(normal_code)
            .stdout(predicate::str::contains("EXTRA"));
        Command::cargo_bin("sspf")
            .unwrap()
            .args([
                "check",
                "--manifest",
                manifest.to_str().unwrap(),
                "--strict-extra",
            ])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("EXTRA"));
    }
}

#[test]
fn claim_exit_codes_describe_pass_drift_and_input_error() {
    // @claim:ci-exit-codes
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("keys"), "EXPECTED\n").unwrap();
    fs::write(
        directory.path().join("pass.toml"),
        manifest_for("keys", &["EXPECTED"], "block"),
    )
    .unwrap();
    Command::cargo_bin("sspf")
        .unwrap()
        .args([
            "check",
            "--manifest",
            directory.path().join("pass.toml").to_str().unwrap(),
        ])
        .assert()
        .code(0);
    Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", "examples/preflight.toml"])
        .assert()
        .code(1);
    Command::cargo_bin("sspf")
        .unwrap()
        .args(["check", "--manifest", "not-a-file.toml"])
        .assert()
        .code(2);
}

#[test]
fn claim_terminal_json_github_and_opt_in_report_are_observable() {
    // @claim:report-formats
    let directory = tempfile::tempdir().unwrap();
    let before = directory_entries(directory.path());
    let binary = Command::cargo_bin("sspf").unwrap().get_program().to_owned();

    Command::new(&binary)
        .current_dir(directory.path())
        .args([
            "check",
            "--manifest",
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("examples/preflight.toml")
                .to_str()
                .unwrap(),
            "--format",
            "terminal",
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("expected keys"))
        .stdout(predicate::str::contains("destination keys"));
    assert_eq!(directory_entries(directory.path()), before);

    let json_output = Command::new(&binary)
        .args([
            "check",
            "--manifest",
            "examples/preflight.toml",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert_eq!(json_output.status.code(), Some(1));
    assert_eq!(
        serde_json::from_slice::<Value>(&json_output.stdout).unwrap()["schema_version"],
        1
    );

    Command::new(&binary)
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
            "::error title=Destination limit exceeded::",
        ));

    let report = directory.path().join("requested-report.json");
    Command::new(&binary)
        .args([
            "check",
            "--manifest",
            "examples/preflight.toml",
            "--report",
            report.to_str().unwrap(),
        ])
        .assert()
        .code(1);
    assert_eq!(
        serde_json::from_str::<Value>(&fs::read_to_string(report).unwrap()).unwrap()
            ["schema_version"],
        1
    );
}

#[test]
fn claim_repository_carries_the_mit_license() {
    // @claim:free-mit
    let metadata = std::process::Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .unwrap();
    assert!(metadata.status.success());
    let metadata: Value = serde_json::from_slice(&metadata.stdout).unwrap();
    assert_eq!(metadata["packages"][0]["license"], "MIT");
    let license = fs::read_to_string("LICENSE").unwrap();
    assert!(license.contains("Permission is hereby granted, free of charge"));
    assert!(license.contains("THE SOFTWARE IS PROVIDED \"AS IS\""));
}
