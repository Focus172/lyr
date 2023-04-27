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
use crate::logger::Logger;
use crate::screen::Screen;
use crate::state::State;
use crossterm::{
    event::{Event, KeyCode},
    execute,
    terminal::EnterAlternateScreen,
};
use std::io;
use std::path::PathBuf;
use tui::{backend::CrosstermBackend, Terminal};

const GIT_VERSION_STRING: &str = "0.1.0";
const HELP_MSG: &str = "Usage: lyr [OPTION]...
  -c, --config=FILE     use FILE as config file
  -h, --help            display this help and exit
  -v, --version         display version and exit";
const DEFAULT_PATH: &str = "/etc/lyr/config.ini";

fn main() -> Result<(), io::Error> {
    let args = std::env::args().collect::<Vec<String>>();

    let mut log = Logger::new();

    let mut config = Config::new();
    parse_args(args, &mut config); //.handle(&mut log);
    config.load().handle(&mut log);

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

    let (mut screen, events) =
        Screen::new(terminal, &mut config).expect("Could not initilize term");

    screen.draw();

    // Place the curser on the login field if there is no saved username, if there is, place the curser on the password field
    // screen.place_cursor(); // config.default_input

    // if config.animate { screen.animate_init(); }

    // the buffers need to go somewhere

    // switch_tty(&buf);

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
            screen.draw();
        }

        // if (config.animate) {
        // 	error = tb_peek_event(&event, config.min_refresh_delta);
        // } else {
        // 	error = tb_poll_event(&event);
        // }

        // if state.error != None {
        // panic!("Some error happened");
        // }

        for event in &events {
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
                        KeyCode::Char('q') => panic!("ended"),
                        KeyCode::Char(c) => {
                            state.append_active(c);
                            state.update = true
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // 		case TB_KEY_CTRL_C:
        // 			run = false;
    }

    screen.close();

    //execl("/bin/sh", "sh", "-c", config.boot_cmd, NULL);
    //execl("/bin/sh", "sh", "-c", config.shutdown_cmd, NULL);
    //execl("/bin/sh", "sh", "-c", config.restart_cmd, NULL);

    Ok(())
}

pub enum Status {
    Ok,
    Info(String),
    Bail(String),
}

impl Status {
    fn handle(&self, log: &mut Logger) {
        match self {
            Status::Ok => {}
            Status::Info(msg) => {
                eprintln!("{}", msg);
                std::process::exit(0);
            }
            Status::Bail(msg) => {
                // log.log(msg);
                eprintln!("{}", msg);
                std::process::exit(1);
            }
        }
    }
}

fn parse_args(mut args: Vec<String>, conf: &mut Config) -> Status {
    while !args.is_empty() {
        let arg = args.remove(0);
        match arg.as_str() {
            "--config" | "-c" => conf.config_path = Some(PathBuf::from(args.remove(0))),
            "--help" | "-h" => return Status::Info(format!("{HELP_MSG}")),
            "--version" | "-v" => return Status::Info(format!("Ly version {GIT_VERSION_STRING}")),
            _ => return Status::Bail(format!("Unknown argument: {arg}")),
        }
    }
    Status::Ok
}
