use std::{
    io::{self, Stdout},
    sync::mpsc::{self, Receiver},
    thread,
};
use crossterm::{
    event::Event,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    // layout::{Constraint, Direction, Layout},
    // widgets::{Block, Borders, Paragraph},
    // Frame,
    Terminal,
};
use crate::{config::Config, input::Input, state::{State, SelectedFeild}};

pub struct Screen {
    term: Terminal<CrosstermBackend<Stdout>>,
    // curser: (u16, u16),
    // current_box: Rect,
    show_help: bool,
}

impl Screen {
    pub fn new(conf: &Config) -> Result<(Screen, Receiver<Event>), std::io::Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        let (tx, rx) = mpsc::channel();

        // start reading events asyncronosly
        let input = Input::new(tx);
        thread::spawn(move || input.read_events());

        // let buf = Buffer::new();

        // let buf.height = self.term_buf.height
        // let buf.height = self.term_buf.height

        // hostname(&buf->info_line);

        // let max_len_login = lang.login.len();
        // let max_len_password = lang.password.len();

        // let max_len = max(max_len_login, max_len_password);

        // let box_height = 7 + (2 * conf.margin_box_v);
        // let box_width = (2 * conf.margin_box_h) + (config.input_len + 1) + max_len;

        // let box_width = 24;
        // let box_height = 8;

        // buf->box_chars.left_up = 0x250c;
        // buf->box_chars.left_down = 0x2514;
        // buf->box_chars.right_up = 0x2510;
        // buf->box_chars.right_down = 0x2518;
        // buf->box_chars.top = 0x2500;
        // buf->box_chars.bot = 0x2500;
        // buf->box_chars.left = 0x2502;
        // buf->box_chars.right = 0x2502;

        Ok((
            Screen {
                term: terminal,
                // curser: (0, 0),
                // current_box: Rect::new(0, 0, 0, 0),
                // use this as a way to only recalculate the rect when the terminal is resized
                // as this is in a tty this is a meaningfull assumption
                show_help: conf.show_fkeys,
            },
            rx,
        ))
    }

    // TODO this should get the currently displayed text from the main thread
    pub fn draw(&mut self, state: &State) -> Result<(), std::io::Error> {

// log: &mut File)

        // log.write("starting write".as_bytes())?;

        self.term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                // .margin(1)
                .constraints(
                    [
                        Constraint::Length(1), // status bar
                        Constraint::Min(1), // the rest
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let status_line = chunks[0];

            let div = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Min(6),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);

            let main_box = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),
                    // Constraint::Percentage(60),
                    Constraint::Min(6),
                    Constraint::Percentage(20),
                    ].as_ref()
                )
                .split(div[1])[1];

            if self.show_help {
                let text = Text::from(Spans::from(vec![
                    Span::raw("Reboot: "),
                    Span::styled("F1", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(", Shutdown: "),
                    Span::styled("F2", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(", Capslock: todo! "),
                    Span::raw("Renders: "),
                    Span::raw(state.renders.to_string()),
                ]));

                // text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

                let help_message = Paragraph::new(text);
                f.render_widget(help_message, status_line);
            }

            // let desktop = Paragraph::new("this is where the current boot os would go");
            // f.render_widget(desktop, chunks[1]);

            // let name = Block::default().title("Hostname").borders(Borders::ALL);
            // f.render_widget(name, main_box);

            // log.write("Wrote box".as_bytes()).unwrap();

            // let pass = Block::default().title("Password").borders(Borders::ALL);
            // f.render_widget(pass, chunks[3]);

            let desktop_text = vec![
                Spans::from(vec![
                    Span::raw("< "),
                    Span::styled(state.data.desktop.display.clone(), Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" >"),
                ]),
                Spans::from(vec![
                    Span::raw("Username: "),
                    Span::raw(state.data.name.word.clone()),
                ]),
                Spans::from(vec![
                    Span::raw("Password: "),
                    Span::raw(state.data.pass.word.clone()),
                ]),
            ];
             
            let desktop = Paragraph::new(desktop_text).block(
                Block::default()
                    .title("Hostname")
                    .borders(Borders::ALL)
                );
                // .alignment(Alignment::Center);


            f.render_widget(desktop, main_box);

            f.set_cursor(main_box.x + 1 + get_x_offset(&state), main_box.y + 1 + get_y_offset(&state));

        })?;

        // log.write("finished write".as_bytes()).unwrap();

        //     let input = Paragraph::new(app.input.as_ref())
        //         .style(InputMode::Editing => Style::default().fg(Color::Yellow))
        //         .block(Block::default().borders(Borders::ALL).title("Input"));
        //     f.render_widget(input, chunks[1]);
        //
        //             // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        //             f.set_cursor(
        //                 // Put cursor past the end of the input text
        //                 chunks[1].x + app.input.len() as u16 + 1,
        //                 // Move one line down, from the border to the input line
        //                 chunks[1].y + 1,
        //             )
        //
        //     let messages: Vec<ListItem> = app
        //         .messages
        //         .iter()
        //         .enumerate()
        //         .map(|(i, m)| {
        //             let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
        //             ListItem::new(content)
        //         })
        //         .collect();
        //     let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        //     f.render_widget(messages, chunks[2]);
        // }

        Ok(())
    }

    pub fn close(&mut self) -> Result<(), std::io::Error> {
        // restore terminal
        disable_raw_mode()?;
        crossterm::execute!(
            self.term.backend_mut(),
            LeaveAlternateScreen,
        )?;
        self.term.show_cursor()?;
        Ok(())
    }
}

fn get_x_offset(s: &State) -> u16 {
    match s.data.selected {
        SelectedFeild::Desktop => 2,
        SelectedFeild::Username => {
            10 + s.data.name.cursor as u16
        }, 
        SelectedFeild::Password =>  {
            10 + s.data.pass.cursor as u16 
        }
    }
}

fn get_y_offset(s: &State) -> u16 {
    match s.data.selected {
        SelectedFeild::Desktop => 0,
        SelectedFeild::Username => 1, 
        SelectedFeild::Password => 2 
    }
}

