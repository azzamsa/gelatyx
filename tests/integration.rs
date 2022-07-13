use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() {
    let mut cmd = Command::cargo_bin("gelatyx").unwrap();
    cmd.arg("-h");
    cmd.assert().success().stdout(predicate::str::contains(
        "Gelatyx 🦤 Format codebease inside the docs",
    ));
}

#[test]
fn file_not_found() {
    let mut cmd = Command::cargo_bin("gelatyx").unwrap();
    cmd.arg("-f").arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn format_file() {
    let mut cmd = Command::cargo_bin("gelatyx").unwrap();
    cmd.arg("-f").arg("tests/fixtures/test.md");
    cmd.assert().success();
}
