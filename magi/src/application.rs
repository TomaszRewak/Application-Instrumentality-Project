use super::instance::Instance;
use super::process::Process;
use std::process::Command;

pub struct Application {
    name: String,

    path: String,
    args: String,

    instances: Vec<Instance>,
}

impl Application {
    pub fn new(name: &str, path: &str, args: &str) -> Application {
        Application {
            name: name.to_string(),
            path: path.to_string(),
            args: args.to_string(),
            instances: Vec::new(),
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
