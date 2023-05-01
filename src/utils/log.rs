use chrono;

pub struct Log {
    name: String
}

impl Log {
    // Creates a new Log instance with a new name.
    pub fn new(name: String) -> Self {
        Self { name }
    }

    // Logs into std 
    pub fn out<T: std::fmt::Display>(&self, msg: T) {
        println!("[{} . {}] {}", self.name, chrono::offset::Local::now(), msg);
    }

    // Panics into std
    pub fn panic<T: std::fmt::Display>(&self, msg: T) {
        panic!("[{} .  {}] {}", self.name, chrono::offset::Local::now(), msg);
    }
}