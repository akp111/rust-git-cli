use std::process::{Command, Stdio};

pub fn git_add() {
    let add_status = Command::new("git")
        .args(["add", "-A"])
        .output()
        .expect("Something went wrong while adding");
}

pub fn git_commit(message: String) {
    let commit_status = Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .expect("Something went wrong while commiting");
    println!("{:?}", commit_status);
    println!("{:?}", commit_status.status.success());
}

pub fn git_push(branch: String) {
    let push_status = Command::new("git")
        .args(["push", "origin", &branch])
        .output()
        .expect("Something went wrong while pushing");
}
