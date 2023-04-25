extern crate tui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, io::Write, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame, Terminal,
};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block = Block::default().title("Binary").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Input").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    let block = Paragraph::new("Words and text and things that are testing thing. Words and text and things that are testing thing. ").block(
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
    );

    f.render_widget(block, chunks[2]);
}

fn main() -> Result<(), io::Error> {
    // setting up the terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    enable_raw_mode()?;
    terminal.hide_cursor()?;

    terminal.draw(|f| {
        ui(f);
        /*
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
        */
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    /*
    let mut res: Option<i64> = Option::None;
    while res.is_none() {
        println!("Welcome to a this program that convers things");
        println!("Choose an options:");
        println!("1. Convert from Binary to Decimal");
        println!("2. Convert from Decimal to Binary");
        print!("Enter your choice: "); std::io::stdout().flush().unwrap();

        let mut buffer = String::new();
        let _ = std::io::stdin().read_line(&mut buffer);

        //checks if the input is a number
        let num = buffer.trim().parse::<i64>();

        match num {
            Ok(number) => res = Some(number),
            Err(_) => {
                println!("Invalid choice")
            }
        }
    }

    let num = res.unwrap();

    match num {
        1 => to_binary(),
        2 => to_decimal(),
        _ => {
            println!("Invalid choice");
            main();
        },
    };
    */

    Ok(())
}

fn to_decimal() {
    println!("You chose to convert from Decimal to Binary");
    print!("Enter a decimal number: ");
    std::io::stdout().flush().unwrap();

    let mut dec_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut dec_buffer);
    let mut dec = match dec_buffer.trim().parse::<i128>() {
        Ok(number) => number,
        Err(_) => {
            println!("Do better");
            to_decimal();
            return;
        }
    };

    let mut bin = String::new();
    while dec > 0 {
        if dec % 2 == 0 {
            bin.push('0');
        } else {
            bin.push('1');
        }
        dec /= 2;
    }
    println!("Out: {}", bin.chars().rev().collect::<String>());
}

fn to_binary() {
    println!("You chose to convert from Binary to Decimal");
    print!("Enter a binary number: ");
    std::io::stdout().flush().unwrap();

    let mut bin_buffer = String::new();
    let _ = std::io::stdin().read_line(&mut bin_buffer);
    let bin: &str = bin_buffer.trim();

    let mut dec: i128 = 0;
    let mut count = 0;

    for c in bin.chars().rev() {
        match c {
            '0' => (),
            '1' => dec += 2_i128.pow(count),
            _ => {
                println!("Invalid binary number, try again");
                to_binary();
                return;
            }
        }
        count += 1;
    }
    println!("Out: {}", dec);
}
