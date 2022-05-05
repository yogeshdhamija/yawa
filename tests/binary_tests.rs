use assert_cmd::assert::Assert;
use assert_cmd::crate_name;
use assert_cmd::Command;
use predicates::str::contains;
use std::fs::{create_dir_all, remove_dir_all};

#[test]
fn displays_help() {
    assert("-h", "", 0).success().stdout(contains("USAGE"));
}

#[test]
fn displays_version() {
    assert("-V", "", 0).success();
}

#[test]
fn fails_with_random_args() {
    assert("random", "", 0).failure();
}

#[test]
fn starts_program() {
    clean(1);
    assert("status", "", 1)
        .failure()
        .stderr(contains("Start a lifting program first!"));
    assert("start -r 105", "", 1)
        .success()
        .stdout(contains("Started program: GZCL-based 4-day cycle"));
    assert("status", "", 1)
        .success()
        .stdout(contains("Current program: GZCL-based 4-day cycle\n"))
        .stdout(contains("Current reference weight: 105\n"))
        .stdout(contains("Starting reference weight: 105\n"))
        .stdout(contains("Workouts completed: 0\n"));
}

#[test]
fn starting_program_needs_reference_weight() {
    clean(2);
    assert("start", "", 2)
        .failure()
        .stderr(contains("REFERENCE_WEIGHT"));
}

#[test]
fn completes_workout() {
    clean(3);
    assert("complete", "", 3)
        .failure()
        .stderr(contains("Start a lifting program first!"));
    assert("start -r 100", "", 3);
    assert("complete", "n\nn\nn\nn\nn\n", 3)
        .success()
        .stdout(contains("Well done!"));
    assert("next", "", 3)
        .success()
        .stdout(contains("Day: Push"))
        .stdout(contains("Bench press"));
}

#[test]
fn prints_next_workout() {
    clean(4);
    assert("next", "", 4)
        .failure()
        .stderr(contains("Start a lifting program first!"));
    assert("start -r 100", "", 4);
    assert("next", "", 4)
        .success()
        .stdout(contains("Weighted Pullup -> 4x3,1x3+ @ 20"));
}

fn assert(args: &str, std_in: &str, test_number: usize) -> Assert {
    let path = dir(test_number);
    create_dir_all(&path).unwrap();
    Command::cargo_bin(crate_name!())
        .unwrap()
        .current_dir(&path)
        .args(args.split_whitespace())
        .write_stdin(std_in)
        .assert()
}

fn dir(test_number: usize) -> String {
    format!("/tmp/yawa/testing/{}", test_number)
}

fn clean(test_number: usize) {
    let path = format!("/tmp/yawa/testing/{}", test_number);
    remove_dir_all(path).unwrap();
}
