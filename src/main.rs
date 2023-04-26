extern crate crossterm;
extern crate tui;

//mod draw;
//mod inputs;
//mod login;
mod utils;
mod config;
// mod logger;
//mod parser;
mod screen;
mod state;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use std::{io, path::PathBuf, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use crate::state::State;
use crate::config::Config;
// use crate::logger::Logger;
use crate::screen::Screen;

const ARG_COUNT: u8 = 7;
const GIT_VERSION_STRING: &str = "0.1.0";
const HELP_MSG: &str = "Usage: lyr [OPTION]...
  -c, --config=FILE     use FILE as config file
  -h, --help            display this help and exit
  -v, --version         display version and exit";
const DEFAULT_PATH: &str = "/etc/lyr/config.ini";

enum End {
    Boot,
    Reboot,
    Shutdown,
}

// enum Status {
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
//                 println!("{}", msg);
//                 std::process::exit(0);
//             }
//             Status::Bail(msg) => {
//                 log.log(msg);
//                 println!("{}", msg);
//                 std::process::exit(1);
//             }
//         }
//     }
// }
//
// fn parse_args(mut args: Vec<String>, conf: &mut Config) -> Status {
//     while !args.is_empty() {
//         let arg = args.remove(0);
//         match arg.as_str() {
//             "--config" | "-c" => conf.config_path = Some(PathBuf::from(args.remove(0))),
//             "--help" | "-h" => return Status::Info(format!("{HELP_MSG}")),
//             "--version" | "-v" => return Status::Info(format!("Ly version {GIT_VERSION_STRING}")),
//             _ => return Status::Bail(format!("Unknown argument: {arg}")),
//         }
//     }
//     Status::Ok
// }

fn main() -> Result<(), io::Error> {
    // let args = std::env::args().collect::<Vec<String>>();

    // let mut log = Logger::new();
    // let mut config = Config::new();

    // parse_args(args, &mut config).handle(&mut log);

    // config.load().handle(&mut log);

    // create 3 buffers with initial values from config
    let buffers = [String::new(), String::new(), String::new()];

    /*
    let inputs = [
        Input::new(&buffers[0], config.login_input),
        Input::new(&buffers[1], config.password_input),
        Input::new(&buffers[2], config.desktop_input),
    ];
    */

    // lazy load desktop or something idk

    // start tui
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal:  = Terminal::new(backend)?;


    // These object are all owned by main who over sees them
    // the events tell main what is happening, it stores these 
    // temporaryly in state then tells the screen to render it
    let mut state = State::new();

    let screen = Screen::new(ui);
    let events = screen.events();
    screen.draw();


    // Place the curser on the login field if there is no saved username, if there is, place the curser on the password field
    screen.place_cursor(); // config.default_input

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
                //update = config.animate;
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

        if state.error != None {
            // well go on do somethign about it
            panic!("Some error happened");
        }
        
        for event in screen.event {
            // do somethign and handle all those key presses
        }

        // 	if (event.type == TB_EVENT_KEY)
        // 	{
        // 		switch (event.key)
        // 		{
        // 		case TB_KEY_F1:
        // 			shutdown = true;
        // 			run = false;
        // 			break;
        // 		case TB_KEY_F2:
        // 			reboot = true;
        // 			run = false;
        // 			break;
        // 		case TB_KEY_CTRL_C:
        // 			run = false;
        // 			break;
        // 		case TB_KEY_CTRL_U:
        // 			if (active_input > 0)
        // 			{
        // 				input_text_clear(input_structs[active_input]);
        // 				update = true;
        // 			}
        // 			break;
        // 		case TB_KEY_ARROW_UP:
        // 			if (active_input > 0)
        // 			{
        // 				--active_input;
        // 				update = true;
        // 			}
        // 			break;
        // 		case TB_KEY_ARROW_DOWN:
        // 			if (active_input < 2)
        // 			{
        // 				++active_input;
        // 				update = true;
        // 			}
        // 			break;
        // 		case TB_KEY_TAB:
        // 			++active_input;

        // 			if (active_input > 2)
        // 			{
        // 				active_input = SESSION_SWITCH;
        // 			}
        // 			update = true;
        // 			break;
        // 		case TB_KEY_ENTER:
        // 			save(&desktop, &login);
        // 			auth(&desktop, &login, &password, &buf);
        // 			update = true;

        // 			if (dgn_catch())
        // 			{
        // 				++auth_fails;
        // 				// move focus back to password input
        // 				active_input = PASSWORD_INPUT;

        // 				if (dgn_output_code() != DGN_PAM)
        // 				{
        // 					buf.info_line = dgn_output_log();
        // 				}

        // 				if (config.blank_password)
        // 				{
        // 					input_text_clear(&password);
        // 				}

        // 				dgn_reset();
        // 			}
        // 			else
        // 			{
        // 				buf.info_line = lang.logout;
        // 			}

        // 			load(&desktop, &login);
        // 			system("tput cnorm");
        // 			break;
        // 		default:
        // 			(*input_handles[active_input])(
        // 				input_structs[active_input],
        // 				&event);
        // 			update = true;
        // 			break;
        // 		}
        // 	}
        // }
    }

    screen.close();

    match state.end {
        End::Boot => {
            //execl("/bin/sh", "sh", "-c", config.boot_cmd, NULL);
        }
        End::Shutdown => {
            //execl("/bin/sh", "sh", "-c", config.shutdown_cmd, NULL);
        }
        End::Reboot => {
            //execl("/bin/sh", "sh", "-c", config.restart_cmd, NULL);
        }
        _ => {}
    };

    Ok(())
}

