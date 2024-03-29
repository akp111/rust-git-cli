use std::process::Command;
use colored::Colorize;

pub fn git_add() {
    let add_status = Command::new("git")
        .args(["add", "-A"])
        .output()
        .expect("Something went wrong while adding");
    if !add_status.status.success() {
        panic!("{:?}", String::from_utf8(add_status.stderr).unwrap());
    }
    println!("{}", "Successfully staged".bright_yellow());
}

pub fn git_pull(branch: &str) {
    let pull_status = Command::new("git")
        .args(["pull", "origin", branch])
        .output()
        .expect("Something went wrong while pulling");
    if !pull_status.status.success() {
        panic!("{:?}", String::from_utf8(pull_status.stderr).unwrap());
    }
    println!("{0} {1}", "Successfully pull from the branch: ".bright_yellow(), branch.bright_yellow());
}

pub fn get_current_branch() -> String {
    let branch_name = Command::new("git")
        .args(["name-rev", "--name-only", "HEAD"])
        .output()
        .expect("Something went wrong while commiting");
    if !branch_name.status.success() {
        panic!("{:?}", String::from_utf8(branch_name.stderr).unwrap());
    }
    let branch = String::from_utf8(branch_name.stdout)
        .unwrap()
        .trim()
        .to_owned();
    println!("{0} {1}", "Current branch is:".bright_yellow(), branch.bright_yellow());
    return branch;
}

pub fn git_commit(message: &str) {
    let commit_status = Command::new("git")
        .args(["commit", "-m", &message])
        .output()
        .expect("Something went wrong while commiting");
    if !commit_status.status.success() {
        panic!("{:?}", String::from_utf8(commit_status.stderr).unwrap());
    }
    println!("{0}", "Successfully committed your code!".bright_yellow());
}

pub fn git_push(branch: &str) {
    let push_status = Command::new("git")
        .args(["push", "origin", &branch])
        .output()
        .expect("Something went wrong while pushing");
    if !push_status.status.success() {
        panic!("{:?}", String::from_utf8(push_status.stderr).unwrap());
    }
    println!("{0} {1}","Succesfuuly pushed to the branch: ".bright_green(), branch.yellow());
}
