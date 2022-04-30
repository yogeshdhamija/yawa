use assert_cmd::crate_name;
use assert_cmd::Command;
use predicates::str::contains;
use std::fs::remove_dir_all;
use std::path::Path;

#[test]
fn displays_help() {
    run_and_assert("-h").success().stdout(contains("USAGE"));
}

#[test]
fn displays_version() {
    run_and_assert("-V").success();
}

#[test]
fn fails_with_random_args() {
    run_and_assert("random").failure();
}

#[test]
fn starts_program() {
    clean_slate();
    run_and_assert("status")
        .failure()
        .stderr(contains("No status. Start a program first!"));
    run_and_assert("start -r 100").success();
    run_and_assert("status").success();
}

#[test]
fn starting_program_needs_reference_weight() {
    run_and_assert("start")
        .failure()
        .stderr(contains("REFERENCE_WEIGHT"));
}

fn clean_slate() {
    remove_dir_all(Path::new("/tmp/yawa")).unwrap();
}

fn run_and_assert(args_to_run_with: &str) -> assert_cmd::assert::Assert {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(args_to_run_with.split_whitespace())
        .assert()
}
