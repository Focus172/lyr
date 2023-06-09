mod input;
//mod login;
mod config;
mod log;
mod screen;
mod state;

use crate::{config::Config, log::Logger, screen::Screen, state::State};
use crossterm::event::{Event, KeyCode};
use std::{process, time::Duration};

use anyhow::Result;

// TODO have a more granular update system
// instead of redrawing the whole screen every time
// only redraw the parts that need to be redrawn

fn main() -> Result<()> {
    let mut log = Logger::new()?;

    let config = Config::new();

    log.log("Starting lyr\n")?;

    // state repersent the current state of the program
    // screen is passed this to render
    // main uses the events to update state
    let mut state = State::default();
    let (mut screen, events) = Screen::new(&config)?;

    // TODO have the selected box be owned by a seperate object so
    // it can bet matched on idependently of the state

    // if config.animate { screen.animate_init(); }

    while state.run {
        if state.update {
            // on auth fail
            // update = cascade(&buf, &auth_fails);

            // Place the curser on the login field if there is no saved username
            // if there is, place the curser on the password field
            state.renders += 1;
            log.log(format!("Starting render: {}\n", state.renders).as_str())?;
            screen.draw(&state)?;
            state.update = false;
            // state.update = config.animate;
        }

        // TODO have this only handle the enter key and pass everything else to some other thing

        // this lands at about 60 fps for the animation
        if let Ok(event) = events.recv_timeout(Duration::from_millis(15)) {
            match event {
                Event::Key(kd) => {
                    match kd.code {
                        KeyCode::F(1) => {
                            state.shutdown = true;
                            state.run = false;
                        }
                        KeyCode::F(2) => {
                            state.reboot = true;
                            state.run = false;
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
                            state.handle_enter();
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
                            state.run = false; // HACK for debugging only
                        }
                        KeyCode::Char(c) => {
                            state.append_active(c);
                            state.update = true
                        }
                        KeyCode::Backspace => {
                            state.del_active();
                            state.update = true;
                        }
                        // TODO have a key that manually updates the screen
                        // TODO have a key that manually exits the program
                        _ => {}
                    }
                }
                Event::Mouse(_pos) => {}
                Event::Resize(_x, _y) => state.update = true,
                Event::Paste(_s) => {}
                Event::FocusLost => {} // this seems like a security vuln somehow
                Event::FocusGained => {}
            }
        }

        // log.log("Finished getting keys\n")?;
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
