use std::process::{Command};

fn main() {
    Command::new(r"C:\Windows\System32\notepad.exe")
        .spawn()
        .expect("Failed to execute command");

    println!("Hello, world 1!");
}
