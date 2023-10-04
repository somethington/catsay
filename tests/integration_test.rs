use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;

#[test]
fn run_with_defaults() {
    Command::cargo_bin("catsay")
    .expect("binary exists")
    .assert()
    .success()
    .stdout(predicate::str::contains("Meow!"));

}

fn fail_on_non_existing_file()
    -> Result<(), Box<dyn std::error::Error>> {
        Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "no/such/fule.txt"])
        .assert()
        .failure();
    Ok(())
}