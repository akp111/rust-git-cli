use colored::*;
use std::io;

pub fn get_branch_from_user() -> String {
    // name of the branch, default is main
    println!("{}", "Enter the branch name, default is main".green());
    let default_branch = String::from("main");
    let mut branch = String::new();
    io::stdin()
        .read_line(&mut branch)
        .expect("Failed to read line");
    let mut branch = branch.trim();
    if branch.is_empty() {
        branch = &default_branch;
    }
    return branch.to_string();
}

pub fn get_commit_message_from_user() -> String {
    println!("{}", "Enter the commit message".green());
    // ask the user to get the commit message;
    let mut commit_msg = String::new();
    io::stdin()
        .read_line(&mut commit_msg)
        .expect("Failed to read line");
    let commit_msg = commit_msg.trim();
    if commit_msg.is_empty() {
        panic!("{}", "Commit message cannot be empty".red());
    }
    return commit_msg.to_string();
}
