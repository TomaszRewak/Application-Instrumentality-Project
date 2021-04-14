use super::instance::Instance;

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

    pub fn add_instance(&mut self, name: &str) {
        self.instances.push(Instance::new(name));
    }

    pub fn start(&mut self, instance_name: &str) {
        if let Some(instance) = self.instances.first_mut() {
            instance.start(&self.path, &self.args);
        }
    }
}
