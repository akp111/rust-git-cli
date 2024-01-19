use colored::Colorize;

pub mod git_helper;
pub mod utils_helper;

fn run_git() {
    let branch = git_helper::get_current_branch();
    let commit_message = utils_helper::get_commit_message_from_user();
    git_helper::git_pull(&branch);
    git_helper::git_add();
    git_helper::git_commit(&commit_message);
    git_helper::git_push(&branch);
}
fn main() {
    println!("{}","Welcome to GIT CLI!!".bright_green());
    run_git();
}
