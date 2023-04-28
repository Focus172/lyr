use std::{fs::File, io::Write};

pub struct Logger {
    file: Option<File>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            // file: None,
            file: Some(File::create("/home/focus/code/lyr/temp.log").unwrap()),
        }
    }

    pub fn log(&mut self, msg: &str) -> Result<usize, std::io::Error>{
        if let Some(file) = &mut self.file {
            file.write(msg.as_bytes())
        } else {
            Ok(0)
        }
    }
}

