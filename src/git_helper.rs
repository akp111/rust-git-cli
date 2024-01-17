use std::process::{Command, Stdio};

pub fn git_add() {
    let add_status = Command::new("git")
        .args(["add", "-A"])
        .output()
        .expect("Something went wrong while adding");
    println!("{:?}", add_status.status.success());
}

pub fn git_commit(message: String) {
    let commit_status = Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .expect("Something went wrong while commiting");
    println!("{:?}", commit_status);
    println!("{:?}", commit_status.status.success());
    if !commit_status.status.success() {
        panic!("Not able to commit")
    }
}

pub fn git_push(branch: String) {
    let push_status = Command::new("git")
        .args(["push", "origin", &branch])
        .output()
        .expect("Something went wrong while pushing");
    println!("{:?}", push_status);
    println!("{:?}", push_status.status.success());
    if !push_status.status.success() {
        panic!("Not able to push")
    }
}
