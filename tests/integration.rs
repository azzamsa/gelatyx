use std::path::Path;

use anyhow::Result;
use assert_cmd::prelude::*;
use gelatyx::util::read_file;
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
    cmd.arg("lua").arg("-f").arg(&path);
    cmd.assert().success();

    let content = read_file(&path)?;
    assert!(content.contains(r#"return { foo }"#));
    Ok(())
}

#[test]
fn format_multiple_file() -> Result<()> {
    let mut cmd = Command::cargo_bin("gelatyx")?;
    let fixture_path = Path::new("tests").join("fixtures");
    let md1 = fixture_path.join("test.md");
    let md2 = fixture_path.join("test2.md");
    // Can't use glob here. It doesn't expand automatically
    // such in termninal invocation.
    cmd.arg("lua").arg("-f").arg(&md1).arg(&md2);
    cmd.assert().success();

    let content1 = read_file(&md1)?;
    let content2 = read_file(&md2)?;
    assert!(content1.contains(r#"return { foo }"#));
    assert!(content2.contains(r#"return { foo }"#));

    Ok(())
}
