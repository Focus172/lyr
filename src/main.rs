// extern crate crossterm;
extern crate serde;
// extern crate tui;

mod input;
//mod login;
mod screen;
mod state;
mod config;

use crate::config::Config;
use crate::screen::Screen;
use crate::state::State;
use crossterm::event::{Event, KeyCode};
use std::{io::{self, Write}, process, fs::File, time::Duration};

fn main() -> Result<(), io::Error> {
    // let mut log = Logger::new();

    let config = Config::new();

    // temp file for logging
    let mut file = File::create("/home/focus/code/lyr/temp.log")?;
    file.write("Started logging\n".as_bytes())?;

    
    // lazy load desktop or something idk, this was in orignal and i dont know what it does

    // These object are all owned by main who over sees them
    // the events tell main what is happening, it stores these
    // temporaryly in state then tells the screen to render it
    let mut state = State::new();
    let (mut screen, events) = Screen::new(&config)?;

    // if config.animate { screen.animate_init(); }

    while state.run {
        if state.update {
            // if state.auth_fails < 10 {
            // } else
            //     //usleep(10000);
            //     //update = cascade(&buf, &auth_fails);
            // }

            // Place the curser on the login field if there is no saved username
            // if there is, place the curser on the password field
            state.renders += 1;
            file.write(format!("remder: {}\n", state.renders).as_bytes())?;
            screen.draw(&state, &mut file)?;
            // state.update = config.animate;
        }
    

        // if (config.animate) {
        // 	error = tb_peek_event(&event, config.min_refresh_delta);
        // }
        
        // if state.error != None {
        // panic!("Some error happened");
        // }

        file.write("\n Events: ".as_bytes())?;
        let event = events.recv_timeout(Duration::from_millis(50));

        match event {
            Ok(event) => {
        
            file.write(format!("\n - {:?}", event).as_bytes())?;
            match event {
            Event::Key(kd) => { match kd.code {
                KeyCode::F(1) => {
                    state.shutdown = true;
                    // state.run = false;
                }
                KeyCode::F(2) => {
                    state.reboot = true;
                    // state.run = false;
                }
                KeyCode::Down => {
                    state.next_buffer();
                   state.update = true;
                }
                KeyCode::Up => {
                    state.prev_buffer();
                    state.update = true;
                }
                KeyCode::Tab => {
                    state.handle_tab();
                    state.update = true;
                }
                KeyCode::Enter => {
                    // save the two input feilds
                    // attempt to authenticate

                    // if auth auth
                    // > increment fails by 1
                    // > move input back to password
                    // > display pam message on info line
                    // > clear the password
                    // > reset the authenticate

                    // else
                    // > set into line to logout message?
                    // > load(&desktop, &login);
                    // > system("tput cnorm");
                }
                KeyCode::Char('q') => {
                    // state.run = false
                    panic!("Quit"); 
                },
                KeyCode::Char(c) => {
                    state.append_active(c);
                    state.update = true
                },
                // KeyCode::Backspace => {
                //     app.input.pop();
                // }
                // TODO have a key that manually updates the screen
                _ => {}
            }}
            Event::Mouse(_pos) => {},
            Event::Resize(_x, _y) => {
                state.update = true
            },
            Event::Paste(_s) => {},
            Event::FocusLost => {}, // this seems like a security vuln somehow
            Event::FocusGained => {},
        }
            }
            Err(_) => {},
        }
        // 		case TB_KEY_CTRL_C:
        // 			run = false;

        file.write("finished getting keys".as_bytes())?;
    }

    screen.close()?;

    if state.shutdown {
        //execl("/bin/sh", "sh", "-c", config.shutdown_cmd, NULL);
        println!("shutting down...");
        process::exit(0)
    }
    
    if state.reboot {
        //execl("/bin/sh", "sh", "-c", config.restart_cmd, NULL);
        println!("rebooting...");
        process::exit(0)
    }

    //execl("/bin/sh", "sh", "-c", config.boot_cmd, NULL);

    Ok(())
}


// pub enum Status {
//     Ok,
//     Info(String),
//     Bail(String),
// }
//
// impl Status {
//     fn handle(&self, log: &mut Logger) {
//         match self {
//             Status::Ok => {}
//             Status::Info(msg) => {
//                 eprintln!("{}", msg);
//                 std::process::exit(0);
//             }
//             Status::Bail(msg) => {
//                 log.log(msg);
//                 eprintln!("{}", msg);
//                 std::process::exit(1);
//             }
//         }
//     }
// }

