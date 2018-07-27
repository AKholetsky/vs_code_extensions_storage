use std::process::{Command, Stdio};

fn main() {
    let cmd = "code-insiders.cmd";
    let child = Command::new("cmd").args(&["/C", cmd, "--list-extensions", "--show-versions"]).stdout(Stdio::piped()).spawn().expect("failed to exec");
    let result = child.wait_with_output().expect("faile to wait");
    println!("{}", String::from_utf8(result.stdout).unwrap());
}