mod application;
mod instance;

use application::Application;

fn main() {
    let mut application = Application::new("test", r"C:\Windows\System32\notepad.exe", "");

    application.add_instance("test");
    application.start("test");

    println!("Hello, world 1!");
}
