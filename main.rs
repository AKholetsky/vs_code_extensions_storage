mod plugin;

use std::process::{Command, Stdio, Child};
use plugin::Plugin;
use std::collections::HashMap;

fn main() {
    let result = run_command().wait_with_output().expect("faile to wait");
    let plugins = String::from_utf8(result.stdout).unwrap();
    let mut plugins_map = HashMap::new();
    if !plugins.is_empty() {
        for plugin in plugins.lines() {
            let pl = Plugin::new(plugin);
            plugins_map.insert(pl.author(), pl);
        }
    }
    println!("{:#?}", plugins_map);
}

fn run_command() -> Child {
    let args = ["/C", "code-insiders.cmd", "--list-extensions", "--show-versions"];
    Command::new("cmd")
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to exec")
}