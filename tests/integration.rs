use std::{error::Error, fs, path::Path, process::Command};

use assert_cmd::{crate_name, prelude::*};
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Format codebease inside the docs"));

    Ok(())
}

#[test]
fn missing_lang() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    let path = Path::new("tests").join("doesnt").join("exist");
    cmd.arg("-f").arg(path);
    cmd.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));

    Ok(())
}

#[test]
fn file_not_found() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    let path = Path::new("tests").join("doesnt").join("exist");
    cmd.arg("lua").arg("-f").arg(path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File is not found"));
    Ok(())
}

#[test]
fn format_multiple_file() -> Result<(), Box<dyn Error>> {
    let content = r#"""
# Document Title

first line

```lua
local foo = require("bar")
return { foo }
```

second line
"""#;

    let mut cmd = Command::cargo_bin(crate_name!())?;

    let temp_dir = assert_fs::TempDir::new()?;
    let md1 = temp_dir.child("first.md");
    md1.write_str(content)?;
    let md2 = temp_dir.child("second.md");
    md2.write_str(content)?;

    // Can't use glob here. It doesn't expand automatically
    // such in termninal invocation.
    cmd.arg("lua")
        .arg("-f")
        .arg(md1.to_path_buf())
        .arg(md2.to_path_buf());
    cmd.assert().success();

    let content1 = fs::read_to_string(md1)?;
    assert!(content1.contains(r#"return { foo }"#));
    let content2 = fs::read_to_string(md2)?;
    assert!(content2.contains(r#"return { foo }"#));

    temp_dir.close()?;
    Ok(())
}

#[test]
fn format_nocode_file() -> Result<(), Box<dyn Error>> {
    let content = r#"""
# Document Title

first line

second line
"""#;

    let mut cmd = Command::cargo_bin(crate_name!())?;

    let temp_dir = assert_fs::TempDir::new()?;
    let input = temp_dir.child("nocode.md");
    input.write_str(content)?;

    cmd.arg("lua").arg("-f").arg(input.to_path_buf());
    cmd.assert().success();

    temp_dir.close()?;
    Ok(())
}

#[test]
fn check_file() -> Result<(), Box<dyn Error>> {
    let content = r#"""
# Document Title

first line

```lua
local foo=require("bar")
return {foo}
```

second line
"""#;

    let mut cmd = Command::cargo_bin(crate_name!())?;

    let temp_dir = assert_fs::TempDir::new()?;
    let input = temp_dir.child("check.md");
    input.write_str(content)?;

    cmd.arg("lua")
        .arg("-f")
        .arg(input.to_path_buf())
        .arg("--check");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("is unformatted"));

    temp_dir.close()?;
    Ok(())
}
