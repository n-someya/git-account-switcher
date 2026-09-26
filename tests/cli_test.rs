use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help_succeeds() {
    let mut cmd = Command::cargo_bin("git-account-switcher").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("git-account-switcher"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("apply"))
        .stdout(predicate::str::contains("env"))
        .stdout(predicate::str::contains("doctor"));
}

#[test]
fn test_cli_version_succeeds() {
    let mut cmd = Command::cargo_bin("git-account-switcher").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_subcommands_display_help() {
    for subcmd in &["init", "apply", "env", "doctor"] {
        let mut cmd = Command::cargo_bin("git-account-switcher").unwrap();
        cmd.args([subcmd, "--help"]).assert().success();
    }
}
