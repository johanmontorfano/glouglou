use std::fs;

// Read a file into a `String`
pub fn read_file(path: &str) -> Result<String, ()> {
    let log = super::log::Log::new("fs::read_file".to_string());

    match fs::read(path) {
        Ok(vec) => { 
            match String::from_utf8(vec) {
                Ok(file_content) => Ok(file_content),
                Err(_) => {
                    log.out(format!("Failed to parse file at {}: the content may be corrupted.", path));
                    Err(())
                }
            }
         },
        Err(reason) => {
            log.out(format!("Failed to read file at {}: {}", path, reason.to_string()));
            Err(())
        }
    }
}

// Read a file into `Vec<u8>`
pub fn raw_read_file(path: &str) -> Result<Vec<u8>, ()> {
    let log = super::log::Log::new("fs::read_file".to_string());

    match fs::read(path) {
        Ok(vec) => Ok(vec),
        Err(reason) => {
            log.out(format!("Failed to read file at {}: {}", path, reason.to_string()));
            Err(())
        }
    }
}