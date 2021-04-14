mod application;
mod instance;

use application::Application;

fn main() {
    let mut application = Application::new(
        "test".to_string(),
        r"C:\Windows\System32\notepad.exe".to_string(),
        "".to_string(),
    );

    application.add_instance("test".to_string());
    application.start("test");

    println!("Hello, world 1!");
}
