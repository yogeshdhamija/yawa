use assert_cmd::Command;
use assert_cmd::crate_name;
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

fn run_with(args: &[&str]) -> assert_cmd::assert::Assert {
    Command::cargo_bin(crate_name!()).unwrap().args(args).assert()
}
