use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;

#[test]
fn filters_text_contains() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.log");
    fs::write(&p, "hello\nerror: boom\nok\n").unwrap();

    cargo_bin_cmd!("log-slicer")
        .arg(p)
        .arg("--contains")
        .arg("error")
        .assert()
        .success()
        .stdout(predicate::str::contains("error: boom"));
}

#[test]
fn filters_json_field_equals() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.ndjson");
    fs::write(
        &p,
        r#"{"level":"info","msg":"hi"}
{"level":"error","msg":"boom"}
"#,
    )
    .unwrap();

    cargo_bin_cmd!("log-slicer")
        .arg(p)
        .arg("--input")
        .arg("json")
        .arg("--field")
        .arg("level")
        .arg("--equals")
        .arg("error")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""level":"error""#));
}

#[test]
fn filters_json_by_since_until() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("a.ndjson");
    fs::write(
        &p,
        r#"{"timestamp":"2025-12-15T09:00:00Z","level":"info","msg":"a"}
{"timestamp":"2025-12-15T09:00:01Z","level":"error","msg":"b"}
{"timestamp":"2025-12-15T09:00:02Z","level":"warn","msg":"c"}
"#,
    )
    .unwrap();

    cargo_bin_cmd!("log-slicer")
        .arg(p)
        .arg("--input")
        .arg("json")
        .arg("--since")
        .arg("2025-12-15T09:00:01Z")
        .arg("--until")
        .arg("2025-12-15T09:00:01Z")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""msg":"b""#))
        .stdout(predicate::str::contains(r#""msg":"a""#).not());
}
