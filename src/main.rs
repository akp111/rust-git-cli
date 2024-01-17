use colored::Colorize;

pub mod git_helper;
pub mod util_helper;

fn run_git() {
    let branch = git_helper::get_current_branch();
    let commit_message = util_helper::get_commit_message_from_user();
    git_helper::git_add();
    git_helper::git_commit(&commit_message);
    git_helper::git_push(&branch);
    println!("{0} {1}","Succesfuuly pushed to the ".bright_green(), branch.yellow());

}
fn main() {
    println!("{}","Welcome to GIT CLI!!".bright_green());
    run_git();
}
