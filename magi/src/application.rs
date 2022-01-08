use super::instance::Instance;

pub struct Application {
    path: String,
    args: String,

    instances: Vec<Instance>,
}

impl Application {
    pub fn new(path: String, args: String) -> Application {
        Application {
            path: path,
            args: args,
            instances: Vec::new(),
        }
    }

    pub fn add_instance(&mut self, name: String) {
        self.instances.push(Instance::new(name));
    }

    pub fn start(&mut self, instance_name: String) {
        if let Some(instance) = self.instances.first_mut() {
            instance.start(&self.path, &self.args);
        }
    }
}
