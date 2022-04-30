use assert_cmd::crate_name;
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn displays_help() {
    let assert = run_with(&["-h"]);
    assert.success().stdout(predicate::str::contains("USAGE"));
}

#[test]
fn displays_version() {
    let assert = run_with(&["-V"]);
    assert.success();
}

#[test]
fn fails_with_random_args() {
    let assert = run_with(&["random"]);
    assert.failure();
}

#[test]
fn starts_program() {
    let assert = run_with(&["start", "-r", "100"]);
    assert.success();
}

#[test]
fn starts_program_needs_reference_weight() {
    let assert = run_with(&["start"]);
    assert
        .failure()
        .stderr(predicate::str::contains("REFERENCE_WEIGHT"));
}

fn run_with(args: &[&str]) -> assert_cmd::assert::Assert {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(args)
        .assert()
}
