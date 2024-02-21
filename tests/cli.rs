use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use grrs::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = String::from("test/file/doesnt/exist");
    let error_msg = format!("could not read from file `{}`", file_path);
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg(file_path);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(error_msg));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn grep_check_true() {
    let grep = Grep::new("hello");
    let mut result = Vec::new();

    grep.check("hello", &mut result);

    assert_eq!(result, b"hello\n");
}

#[test]
fn grep_check_false() {
    let grep = Grep::new("bye");
    let mut result = Vec::new();

    grep.check("hello", &mut result);

    assert_eq!(result, b"");
}
