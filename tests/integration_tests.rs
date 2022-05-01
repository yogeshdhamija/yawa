use assert_cmd::crate_name;
use assert_cmd::Command;
use predicates::str::contains;
use std::fs::remove_dir_all;
use std::path::Path;

#[test]
fn run_all() {
    println!("displays_help");
    displays_help();
    println!("displays_version");
    displays_version();
    println!("fails_with_random_args");
    fails_with_random_args();
    println!("starts_program");
    starts_program();
    println!("starting_program_needs_reference_weight");
    starting_program_needs_reference_weight();
    println!("prints_next_workout");
    prints_next_workout();
}

// Tests (tech debt-- make them runnable concurrently and therefore able to have
// their own #[test] annotations. Currently prevented because they all try to
// save to the same file. Will be fixed when file to save becomes configurable)
fn displays_help() {
    fresh_run_and_assert("-h")
        .success()
        .stdout(contains("USAGE"));
}

fn displays_version() {
    fresh_run_and_assert("-V").success();
}

fn fails_with_random_args() {
    fresh_run_and_assert("random").failure();
}

fn starts_program() {
    clean_slate();
    run_and_assert("status")
        .failure()
        .stderr(contains("No status. Start a program first!"));
    run_and_assert("start -r 105").success();
    run_and_assert("status")
        .success()
        .stdout("Current reference weight: 105\n");
}

fn starting_program_needs_reference_weight() {
    fresh_run_and_assert("start")
        .failure()
        .stderr(contains("REFERENCE_WEIGHT"));
}

fn prints_next_workout() {
    clean_slate();
    run_and_assert("next show").failure().stderr(contains(
        "Can't display next workout. Start a program first!",
    ));
    run_and_assert("start -r 100");
    run_and_assert("next show")
        .success()
        .stdout(contains("100"));
}

// Helpers (tech debt-- make them run in parallel)
fn clean_slate() -> bool {
    remove_dir_all(Path::new("/tmp/yawa")).is_ok()
}

fn fresh_run_and_assert(args_to_run_with: &str) -> assert_cmd::assert::Assert {
    clean_slate();
    run_and_assert(args_to_run_with)
}

fn run_and_assert(args_to_run_with: &str) -> assert_cmd::assert::Assert {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(args_to_run_with.split_whitespace())
        .assert()
}
