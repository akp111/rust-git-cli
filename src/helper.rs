use std::process::Command;

pub fn git_add() {
   let add_status = Command::new("git").args(["add", "-A"]).output().expect("Something went wrong while doing 'git add -A'");
   println!("{:?}", add_status.status.success());
   
}
