use std::process::{Command};
use colored::*;

pub fn git_add() {
    let add_status = Command::new("git")
        .args(["add", "-A"])
        .output()
        .expect("Something went wrong while adding");
    if !add_status.status.success() {
        panic!("{:?}", String::from_utf8(add_status.stderr).unwrap().red());
    }
}

pub fn git_commit(message: &str) {
    let commit_status = Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .expect("Something went wrong while commiting");
    if !commit_status.status.success() {
        panic!("{:?}", String::from_utf8(commit_status.stderr).unwrap().red());
    }
}

pub fn git_push(branch: &str) {
    let push_status = Command::new("git")
        .args(["push", "origin", &branch])
        .output()
        .expect("Something went wrong while pushing");
    if !push_status.status.success() {
        println!("{:?}", String::from_utf8(push_status.stderr).unwrap().red());
        panic!();
    }
}
