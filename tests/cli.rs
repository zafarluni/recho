use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn fails_with_no_args() {
    let mut cmd = Command::cargo_bin("recho").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
}
#[test]
fn displays_args_given() {
    let mut cmd = Command::cargo_bin("recho").unwrap();
    cmd.arg("foo").arg("bar").assert().success().stdout(predicate::str::contains("foo"));
}