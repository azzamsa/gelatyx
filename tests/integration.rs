use std::path::Path;

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
fn missing_lang() {
    let mut cmd = Command::cargo_bin("gelatyx").unwrap();
    let path = Path::new("tests").join("doesnt").join("exist");
    cmd.arg("-f").arg(path);
    cmd.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));
}

#[test]
fn file_not_found() {
    let mut cmd = Command::cargo_bin("gelatyx").unwrap();
    let path = Path::new("tests").join("doesnt").join("exist");
    cmd.arg("lua").arg("-f").arg(path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
}

#[test]
fn format_file() {
    let mut cmd = Command::cargo_bin("gelatyx").unwrap();
    let path = Path::new("tests").join("fixtures").join("test.md");
    cmd.arg("lua").arg("-f").arg(path);
    cmd.assert().success();
}
