use super::instance::Instance;

pub struct Application {
    name: String,

    path: String,
    args: String,

    instances: Vec<Instance>,
}

impl Application {
    pub fn new(name: String, path: String, args: String) -> Application {
        Application {
            name: name,
            path: path,
            args: args,
            instances: Vec::new(),
        }
    }

    pub fn add_instance(&mut self, name: String) {
        self.instances.push(Instance::new(name));
    }

    pub fn start(&mut self, instance_name: &str) {
        if let Some(instance) = self.instances.first_mut() {
            instance.start(&self.path, &self.args);
        }
    }
}
