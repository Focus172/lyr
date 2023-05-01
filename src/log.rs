use std::{fs::File, io::Write};

pub struct Logger {
    file: Option<File>,
}

impl Logger {
    pub fn new() -> Logger {
        if std::env::var("LYR_LOG").is_ok() {
            Logger {
                file: Some(File::create("lyr.log").unwrap()),
            }
        } else {
            Logger { file: None }
        }
    }

    pub fn log(&mut self, msg: &str) { 
        if let Some(file) = &mut self.file {
            file.write(msg.as_bytes());
        } 
    }
}
