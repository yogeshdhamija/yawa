use assert_cmd::assert::Assert;
use assert_cmd::crate_name;
use assert_cmd::Command;
use predicates::str::contains;
use rand::random;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Read;
use std::path::Path;

#[test]
fn displays_help() {
    in_clean_dir(|dir| {
        assert("-h", "", dir).success().stdout(contains("USAGE"));
    });
}

#[test]
fn displays_version() {
    in_clean_dir(|dir| {
        assert("-V", "", dir).success();
    });
}

#[test]
fn fails_with_random_args() {
    in_clean_dir(|dir| {
        assert("random", "", dir).failure();
    });
}

#[test]
fn saves_in_desired_folder() {
    in_clean_dir(|dir| {
        assert("status", "", dir)
            .failure()
            .stderr(contains("Start a lifting program first!"));
        assert("-s in/a/nested/folder/ start -r 105", "", dir)
            .success()
            .stdout(contains("Started program: GZCL-based 4-day cycle"));
        assert("status", "", dir)
            .failure()
            .stderr(contains("Start a lifting program first!"));
        assert("-s in/a/nested/folder/ status", "", dir)
            .success()
            .stdout(contains("Current program: GZCL-based 4-day cycle"))
            .stdout(contains("Current reference weight: 105\n"))
            .stdout(contains("Starting reference weight: 105\n"))
            .stdout(contains("Workouts completed: 0\n"));
        assert!(Path::new(&format!("{dir}/in/a/nested/folder/yawa_save_data/info.txt")).is_file());
    });
}

#[test]
fn starts_program() {
    in_clean_dir(|dir| {
        assert("status", "", dir)
            .failure()
            .stderr(contains("Start a lifting program first!"));
        assert("start -r 105", "", dir)
            .success()
            .stdout(contains("Started program: GZCL-based 4-day cycle"));
        assert("status", "", dir)
            .success()
            .stdout(contains("Current program: GZCL-based 4-day cycle"))
            .stdout(contains("Current reference weight: 105\n"))
            .stdout(contains("Starting reference weight: 105\n"))
            .stdout(contains("Workouts completed: 0\n"));
    });
}

#[test]
fn starting_program_needs_reference_weight() {
    in_clean_dir(|dir| {
        assert("start", "", dir)
            .failure()
            .stderr(contains("REFERENCE_WEIGHT"));
    });
}

#[test]
fn completes_workout() {
    in_clean_dir(|dir| {
        assert("complete", "", dir)
            .failure()
            .stderr(contains("Start a lifting program first!"));
        assert("start -r 100", "", dir);
        assert("complete", "n\nn\nn\nn\nn\n", dir)
            .success()
            .stdout(contains("Well done!"));
        assert("next", "", dir)
            .success()
            .stdout(contains("Day: Push"))
            .stdout(contains("Bench press"));

        let mut history_file_data = String::new();
        let mut file =
            File::open(Path::new(&format!("{dir}/yawa_save_data/lift_history.txt"))).unwrap();
        file.read_to_string(&mut history_file_data).unwrap();

        assert!(history_file_data.contains("Barbell Row -> 3x10 @ 65 | NotCompleted\n"));
    });
}

#[test]
fn prints_next_workout() {
    in_clean_dir(|dir| {
        assert("next", "", dir)
            .failure()
            .stderr(contains("Start a lifting program first!"));
        assert("start -r 100", "", dir);
        assert("next", "", dir)
            .success()
            .stdout(contains("Weighted Pullup -> 4x3,1x3+ @ 20"));
    })
}

#[test]
fn works_with_current_major_version_data() {
    in_dir_with_current_major_version_data(|dir| {
        assert("status", "", dir)
            .success()
            .stdout(contains("Workouts completed: 9"));
        assert("next", "", dir)
            .success()
            .stdout(contains("Bench press -> 4x3,1x3+ @ 205"))
            .stdout(contains("Tricep Cable Pressdown -> 2x15,1x15-25 @ 40"));
    })
}

fn assert(args: &str, std_in: &str, in_dir: &str) -> Assert {
    create_dir_all(in_dir).unwrap();
    Command::cargo_bin(crate_name!())
        .unwrap()
        .current_dir(in_dir)
        .args(args.split_whitespace())
        .write_stdin(std_in)
        .assert()
}

fn dir(test_number: usize) -> String {
    format!("/tmp/yawa/testing/{}", test_number)
}

fn clean(dir: &str) {
    let _ = remove_dir_all(dir);
}

fn in_clean_dir<F, R>(test: F) -> R
where
    F: FnOnce(&str) -> R,
{
    let test_number: usize = random();
    let test_dir = dir(test_number);
    clean(&test_dir);
    test(&test_dir)
}
fn in_dir_with_current_major_version_data<F, R>(test: F) -> R
where
    F: FnOnce(&str) -> R,
{
    test("tests/test_assets/major_version_saved_data/")
}
