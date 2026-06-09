#![expect(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "Tests are set up to expect errors and unwrap them for assertions."
)]

use rstest::rstest;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn prints_the_version() {
    // when
    let output = run_pairs(&["--version"]);

    // then
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pairs"));
}

#[test]
fn prints_help() {
    // when
    let output = run_pairs(&["--help"]);

    // then
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pairs"));
    assert!(stdout.contains("PIN"));
}

#[rstest(
    subcommand,
    help_content,
    case::list("list", "remotely existing stashes"),
    case::pop("pop", "pop the remote stash")
)]
fn prints_help_for_given_subcommand(subcommand: &str, help_content: &str) {
    // when
    let output = run_pairs(&[subcommand, "--help"]);

    // then
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(help_content));
}

#[test]
fn fails_on_invalid_subcommand() {
    // when
    let output = run_pairs(&["invalid-command"]);

    // then
    assert!(!output.status.success());
}

#[test]
fn fails_on_non_numeric_pin() {
    // when
    let output = run_pairs(&["abc"]);

    // then
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid pin"));
}

#[test]
fn fails_when_not_inside_git_repository() {
    // given
    let dir = tempfile::tempdir().expect("tempdir").keep();

    // when
    let output = run_pairs_in(&dir, &[]);

    // then
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Not within a git repository"));
}

#[rstest(subcommand, case::list("list"), case::apply("123"), case::pop("pop"))]
fn fails_when_no_remote_configured(subcommand: &str) {
    // given
    let repo = setup_git_repo();
    std::fs::write(repo.join("dirty.txt"), "changes").unwrap();

    // when
    let output = run_pairs_in(&repo, &[subcommand]);

    // then
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No remote named 'origin' found"));
}

#[test]
fn fails_when_no_remote_configured_on_stash() {
    // given
    let repo = setup_git_repo();
    std::fs::write(repo.join("dirty.txt"), "changes").unwrap();

    // when
    let output = run_pairs_in(&repo, &[]);

    // then
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No remote named 'origin' found"));
}

#[test]
fn cannot_stash_without_changes() {
    // given
    let repo = setup_git_repo_with_remote();

    // when
    let output = run_pairs_in(&repo, &[]);

    // then
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("nothing to stash"));
}

#[test]
fn list_returns_empty_when_no_stashes_exist() {
    // given
    let repo = setup_git_repo_with_remote();

    // when
    let output = run_pairs_in(&repo, &["list"]);

    // then
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No remote stashes found"));
}

#[test]
fn fails_on_unknown_pin() {
    // given
    let repo = setup_git_repo_with_remote();

    // when
    let output = run_pairs_in(&repo, &["999"]);

    // then
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown pin: 999"));
}

#[test]
fn fails_on_pop_when_no_stash_exists() {
    // given
    let repo = setup_git_repo_with_remote();

    // when
    let output = run_pairs_in(&repo, &["pop"]);

    // then
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No pins found"));
}

/// Helper to run the `pairs` binary with given args
fn run_pairs(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_pairs"))
        .args(args)
        .output()
        .expect("Failed to execute pairs binary")
}

/// Helper to run `pairs` in a specific working directory
fn run_pairs_in(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_pairs"))
        .args(args)
        .current_dir(dir)
        .output()
        .expect("Failed to execute pairs binary")
}

/// Helper to run a git command in a specific directory
fn git_in(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("Failed to execute git")
}

/// Creates a temporary git repository with an initial commit and returns the path
fn setup_git_repo() -> PathBuf {
    let dir = tempfile::tempdir()
        .expect("Failed to create tempdir")
        .keep();
    git_in(&dir, &["init"]);
    git_in(&dir, &["config", "user.email", "test@test.com"]);
    git_in(&dir, &["config", "user.name", "Test User"]);
    std::fs::write(dir.join("README.md"), "# test").unwrap();
    git_in(&dir, &["add", "."]);
    git_in(&dir, &["commit", "-m", "initial commit"]);
    dir
}

/// Creates a git repo with a bare remote origin and returns the working repo path
fn setup_git_repo_with_remote() -> PathBuf {
    let bare = tempfile::tempdir().expect("tempdir").keep();
    git_in(&bare, &["init", "--bare"]);

    let repo = tempfile::tempdir().expect("tempdir").keep();
    git_in(&repo, &["init"]);
    git_in(&repo, &["config", "user.email", "test@test.com"]);
    git_in(&repo, &["config", "user.name", "Test User"]);
    git_in(&repo, &["remote", "add", "origin", bare.to_str().unwrap()]);
    std::fs::write(repo.join("README.md"), "# test").unwrap();
    git_in(&repo, &["add", "."]);
    git_in(&repo, &["commit", "-m", "init"]);
    git_in(&repo, &["push", "-u", "origin", "main"]);
    repo
}
