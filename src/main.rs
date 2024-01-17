pub mod git_helper;
pub mod util_helper;

fn run_git() {
    let branch = util_helper::get_branch_from_user();
    let commit_message = util_helper::get_commit_message_from_user();
    git_helper::git_add();
    git_helper::git_commit(commit_message);
    git_helper::git_push(branch);
}
fn main() {
    println!("Welcome to GIT CLI!!");
    run_git();
}
