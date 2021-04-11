use std::process::Command;

mod process;

struct Application {
    path: String,
    args: String,
}

impl Application {
    fn spawn() -> Process {
        Process::new()
    }
}
