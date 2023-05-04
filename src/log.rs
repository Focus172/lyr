use anyhow::Result;
use std::{fs::File, io::Write};

pub struct Logger {
    file: Option<File>,
}

impl Logger {
    pub fn new() -> Result<Logger> {
        if std::env::var("LYR_LOG").is_ok() {
            Ok(Logger {
                file: Some(File::create("lyr.log")?)
            })
        } else {
            Ok(Logger { file: None })
        }
    }

    pub fn log(&mut self, msg: &str) -> Result<()> {
        if let Some(file) = &mut self.file {
            file.write(msg.as_bytes()).map(|_| ())?;
        }
        Ok(())
    }
}
