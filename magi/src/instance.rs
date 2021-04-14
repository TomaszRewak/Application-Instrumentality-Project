use uuid::Uuid;

pub struct Instance {
    name: String,
    unique_id: Uuid,
}

impl Instance {
    fn new(name: &str) -> Instance {
        Instance {
            name: name.to_string(),
            unique_id: Uuid::new_v4(),
        }
    }
}
