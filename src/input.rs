/// A simple threded input handler for crossterm events.

use std::sync::mpsc::Sender;
use crossterm::event::{read, Event};

pub struct Input {
    port: Sender<Event>,
}

impl Input {
    pub fn new(msg: Sender<Event>) -> Input {
        Input {
            port: msg,
        }
    }

    pub fn read_events(&self) {
        loop {
            let evt = read(); 
            match evt {
                Ok(e) => match self.port.send(e) {
                    Ok(_) => {},
                    Err(_) => {}, 
                },
                Err(_) => {}
            }
        }
    }
}

// enum DisplayServer {
//     DsWayland,
//     DsShell,
//     DsXinitrc,
//     DsXorg
// }

// struct Text {
    // char* text;
    // char* end;
    // int64_t len;
    // char* cur;
    // char* visible_start;
    // uint16_t visible_len;

    // uint16_t x;
    // uint16_t y;
// }

pub struct Desktop {
// char** list;
// char** list_simple;
// char** cmd;
// display_server: DisplayServer,

    pub display: String,
// uint16_t cur;
// uint16_t len;
// uint16_t visible_len;
// uint16_t x;
// uint16_t y;
}

