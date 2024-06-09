use std::process::{Command, exit};

fn update_commit_push() {
    let add_command = Command::new("git")
    .args(["add", "."])
    .output()
    .expect("Failed to execute git add command");
    
    let commit_command = Command::new("git")
    .args(["commit", "-m", "Made Changes to Repository"])
    .output()
    .expect("Failed to execute git commit command");
    
    let push_command = Command::new("git")
    .args(["push", "origin", "master"])
    .output()
    .expect("Failed to execute git push command");

    if !add_command.status.success() || !commit_command.status.success() || !push_command.status.success() {
        eprintln!("Failed to update repository");
        exit(1);
    }

    println!("Updated repository");
}

fn main() {
    update_commit_push();
    println!("Done!");
}