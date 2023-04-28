extern crate crossterm;
extern crate tui;

mod inputs;
//mod login;
// mod utils;
mod config;
mod logger;
//mod parser;
mod screen;
mod state;

use crate::config::Config;
use crate::screen::Screen;
use crate::state::State;
use crossterm::{
    event::{Event, KeyCode},
    execute,
    terminal::EnterAlternateScreen,
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

const DEFAULT_PATH: &str = "/etc/lyr/config.ini";

fn main() -> Result<(), io::Error> {
    // let mut log = Logger::new();

    let config = Config::new()?;

    // lazy load desktop or something idk, this was in orignal and i dont know what it does

    // start tui
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    // These object are all owned by main who over sees them
    // the events tell main what is happening, it stores these
    // temporaryly in state then tells the screen to render it
    let mut state = State::new();

    let (mut screen, events) = Screen::new(terminal, &config)?;


    // Place the curser on the login field if there is no saved username
    // if there is, place the curser on the password field
    screen.draw(&state)?;

    // if config.animate { screen.animate_init(); }

    while state.run {
        if state.update {
            if state.auth_fails < 10 {
                //(*input_handles[active_input])(input_structs[active_input], NULL);
                //tb_clear();
                //animate(&buf);
                //draw_box(&buf);
                //draw_labels(&buf);
                //if(!config.hide_f1_commands)
                //	draw_f_commands();
                //draw_lock_state(&buf);
                //position_input(&buf, &desktop, &login, &password);
                //draw_desktop(&desktop);
                //draw_input(&login);
                //draw_input_mask(&password);
                // state.update = config.animate;
            } else {
                //usleep(10000);
                //update = cascade(&buf, &auth_fails);
            }
            screen.draw(&state)?;
        }

        // if (config.animate) {
        // 	error = tb_peek_event(&event, config.min_refresh_delta);
        // } else {
        // 	error = tb_poll_event(&event);
        // }

        // if state.error != None {
        // panic!("Some error happened");
        // }

        for event in &events { match event {
            Event::Key(kd) => { match kd.code {
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
                    // if state.active_input == 0 {
                    // cycle the selected desktop thing
                    // } else {
                    // go to the next feild without wrapping
                    // }
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
                KeyCode::Char('q') => panic!("Quit"), 
                KeyCode::Char(c) => {
                    state.append_active(c);
                    state.update = true
                },
//                     KeyCode::Backspace => {
//                         app.input.pop();
//                     }

                _ => {}
            }}
            Event::Mouse(_pos) => {},
            Event::Resize(_x, _y) => {},
            Event::Paste(_s) => {},
            Event::FocusLost => {}, // this seems like a security vuln somehow
            Event::FocusGained => {},
        }}
        // 		case TB_KEY_CTRL_C:
        // 			run = false;
    }

    screen.close();

    //execl("/bin/sh", "sh", "-c", config.boot_cmd, NULL);
    //execl("/bin/sh", "sh", "-c", config.shutdown_cmd, NULL);
    //execl("/bin/sh", "sh", "-c", config.restart_cmd, NULL);

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

