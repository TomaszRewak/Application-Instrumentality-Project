mod application;
mod instance;
mod process;

use application::Application;

fn main() {
    let application = Application::new(r"C:\Windows\System32\notepad.exe", "");

    application.spawn();

    println!("Hello, world 1!");
}
