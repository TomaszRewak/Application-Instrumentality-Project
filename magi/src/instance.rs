use std::process::Command;
use uuid::Uuid;

pub struct Instance {
    name: String,
    unique_id: Uuid,

    process: Option<std::process::Child>,
}

impl Instance {
    pub fn new(name: &str) -> Instance {
        Instance {
            name: name.to_string(),
            unique_id: Uuid::new_v4(),
            process: None,
        }
    }

    pub fn start(&mut self, path: &str, args: &str) {
        let args = args.replace("<UNIQUE_INSTANCE_ID>", &self.unique_id.to_string());
        let args = args.replace("<INSTANCE_NAME>", &self.name);

        let process = Command::new(path)
            .arg(args)
            .spawn()
            .expect("Failed to execute command");

        self.process = Some(process);
    }
}
