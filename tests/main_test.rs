use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use std::io::{Write};
use tempfile::NamedTempFile;

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    minigrep::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn does_not_find_a_match() {
    let mut result = Vec::new();
    minigrep::find_matches("lorem ipsum\ndolor sit amet", "llama", &mut result);
    assert_eq!(result, b"");
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("minigrep")?;
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("minigrep")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

