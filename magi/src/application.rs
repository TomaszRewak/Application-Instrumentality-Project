use super::process::Process;
use std::process::Command;

pub struct Application {
    path: String,
    args: String,
}

impl Application {
    pub fn new(path: &str, args: &str) -> Application {
        Application {
            path: path.to_string(),
            args: args.to_string(),
        }
    }

    pub fn spawn(&self) -> Process {
        let process = Command::new(self.path.clone())
            .arg(self.args.clone())
            .spawn()
            .expect("Failed to execute command");

        Process::new(process)
    }
}
