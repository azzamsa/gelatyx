use std::path::Path;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help() -> Result<()> {
    let mut cmd = Command::cargo_bin("gelatyx")?;
    cmd.arg("-h");
    cmd.assert().success().stdout(predicate::str::contains(
        "Gelatyx 🦤 Format codebease inside the docs",
    ));

    Ok(())
}

#[test]
fn missing_lang() -> Result<()> {
    let mut cmd = Command::cargo_bin("gelatyx")?;
    let path = Path::new("tests").join("doesnt").join("exist");
    cmd.arg("-f").arg(path);
    cmd.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));

    Ok(())
}

#[test]
fn file_not_found() -> Result<()> {
    let mut cmd = Command::cargo_bin("gelatyx")?;
    let path = Path::new("tests").join("doesnt").join("exist");
    cmd.arg("lua").arg("-f").arg(path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file"));
    Ok(())
}

#[test]
fn format_file() -> Result<()> {
    let mut cmd = Command::cargo_bin("gelatyx")?;
    let path = Path::new("tests").join("fixtures").join("test.md");
    cmd.arg("lua").arg("-f").arg(path);
    cmd.assert().success();
    Ok(())
}
