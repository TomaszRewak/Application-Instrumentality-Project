pub struct Process {
    process: std::process::Child,
}

impl Process {
    pub fn new(process: std::process::Child) -> Process {
        Process { process: process }
    }
}
