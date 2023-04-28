use std::{
    io::{self, Stdout},
    sync::mpsc::Receiver,
    thread,
};

use crossterm::{event::{Event, EnableMouseCapture, DisableMouseCapture}, terminal::{self, enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}};
// use crossterm::{
// execute,
// terminal::{enable_raw_mode, EnterAlternateScreen},
// };
// use std::{io, path::PathBuf, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    // layout::{Constraint, Direction, Layout},
    // widgets::{Block, Borders, Paragraph},
    // Frame,
    Terminal, text::{Span, Text, Spans}, style::{Style, Modifier},
};

pub struct Screen {
    term: Terminal<CrosstermBackend<Stdout>>,
    width: i32,
    height: i32,
}

// struct Box {
// left_up: u32,
// uint32_t left_down;
// uint32_t right_up;
// uint32_t right_down;
// uint32_t top;
// uint32_t bot;
// uint32_t left;
// uint32_t right;
// }

// struct DoomState {
// 	buf: String,
// }
//
// struct AnimationState {
// 	doom: DoomState,
// 	matrix: MatrixState,
// }

struct Term {
    // uint16_t width;
    // uint16_t height;
    // uint16_t init_width;
    // uint16_t init_height;
    //
    // struct box box_chars;
    // char* info_line;
    // uint16_t labels_max_len;
    // uint16_t box_x;
    // uint16_t box_y;
    // uint16_t box_width;
    // uint16_t box_height;

    // union anim_state astate;
}

use std::sync::mpsc;

use crate::{config::Config, input::Input, state::State};

impl Screen {
    pub fn new(conf: &Config) -> Result<(Screen, Receiver<Event>), std::io::Error> {
        
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        let (tx, rx) = mpsc::channel();

        // start reading events asyncronosly
        let input = Input::new(std::io::stdin(), tx);
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

        let box_width = 24;
        let box_height = 8;

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
                width: box_width,
                height: box_height,
            },
            rx,
        ))
    }


    // TODO this should get the currently displayed text from the main thread
    pub fn draw(&mut self, state: &State) -> Result<(), std::io::Error>{
        
        self.term.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(1), // status bar
                    Constraint::Length(1), // current os
                    Constraint::Length(3), // username
                    Constraint::Length(3), // password
                    Constraint::Min(1),
                    // Constraint::Percentage(10),
                ].as_ref())
                .split(f.size());


            // (msg, style)

            let text = Text::from(Spans::from(vec![
                Span::raw("Reboot: "),
                Span::styled("F1", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(", Shutdown: "),
                Span::styled("F2", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(", Capslock: todo!."),
            ]));

            // text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));

            let help_message = Paragraph::new(text);
            f.render_widget(help_message, chunks[0]);

            let desktop = Paragraph::new("this is where the current boot os would go");
            f.render_widget(desktop, chunks[1]);

            let name = Block::default().title("Username").borders(Borders::ALL);
            f.render_widget(name, chunks[2]);

            let pass = Block::default().title("Password").borders(Borders::ALL);
            f.render_widget(pass, chunks[3]);

            // let _block = Paragraph::new("Words and text and things that are testing thing. Words and text and things that are testing thing. ").block(
            // Block::default()
            //     .title("Block")
            //     .borders(Borders::ALL));

            // let size = f.size();
            //    let block = Block::default()
            //        .title("Block")
            //        .borders(Borders::ALL);
            //    f.render_widget(block, size);

        })?;



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

            //     // create app and run it
//     let app = App::default();
//     let res = run_app(&mut terminal, app);
//
//     if let Err(err) = res {
//         println!("{:?}", err)
//     }
//
//     Ok(())
// }

    }
    pub fn close(&mut self) -> Result<(), std::io::Error> {
        // restore terminal
        disable_raw_mode()?;
        crossterm::execute!(
            self.term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.term.show_cursor()?;
        Ok(())
    }
}

// static void doom_free(struct term_buf* buf);
// static void matrix_free(struct term_buf* buf);

// void draw_free(struct term_buf* buf)
// {
// 	if (config.animate)
// 	{
// 		switch (config.animation)
// 		{
// 			case 0:
// 				doom_free(buf);
// 				break;
// 			case 1:
// 				matrix_free(buf);
// 				break;
// 		}
// 	}
// }

// void draw_box(struct term_buf* buf)
// {
// 	uint16_t box_x = (buf->width - buf->box_width) / 2;
// 	uint16_t box_y = (buf->height - buf->box_height) / 2;
// 	uint16_t box_x2 = (buf->width + buf->box_width) / 2;
// 	uint16_t box_y2 = (buf->height + buf->box_height) / 2;
// 	buf->box_x = box_x;
// 	buf->box_y = box_y;
//
// 	if (!config.hide_borders)
// 	{
// 		// corners
// 		tb_change_cell(
// 			box_x - 1,
// 			box_y - 1,
// 			buf->box_chars.left_up,
// 			config.fg,
// 			config.bg);
// 		tb_change_cell(
// 			box_x2,
// 			box_y - 1,
// 			buf->box_chars.right_up,
// 			config.fg,
// 			config.bg);
// 		tb_change_cell(
// 			box_x - 1,
// 			box_y2,
// 			buf->box_chars.left_down,
// 			config.fg,
// 			config.bg);
// 		tb_change_cell(
// 			box_x2,
// 			box_y2,
// 			buf->box_chars.right_down,
// 			config.fg,
// 			config.bg);
//
// 		// top and bottom
// 		struct tb_cell c1 = {buf->box_chars.top, config.fg, config.bg};
// 		struct tb_cell c2 = {buf->box_chars.bot, config.fg, config.bg};
//
// 		for (uint16_t i = 0; i < buf->box_width; ++i)
// 		{
// 			tb_put_cell(
// 				box_x + i,
// 				box_y - 1,
// 				&c1);
// 			tb_put_cell(
// 				box_x + i,
// 				box_y2,
// 				&c2);
// 		}
//
// 		// left and right
// 		c1.ch = buf->box_chars.left;
// 		c2.ch = buf->box_chars.right;
//
// 		for (uint16_t i = 0; i < buf->box_height; ++i)
// 		{
// 			tb_put_cell(
// 				box_x - 1,
// 				box_y + i,
// 				&c1);
//
// 			tb_put_cell(
// 				box_x2,
// 				box_y + i,
// 				&c2);
// 		}
// 	}
//
// 	if (config.blank_box)
// 	{
// 		struct tb_cell blank = {' ', config.fg, config.bg};
//
// 		for (uint16_t i = 0; i < buf->box_height; ++i)
// 		{
// 			for (uint16_t k = 0; k < buf->box_width; ++k)
// 			{
// 				tb_put_cell(
// 					box_x + k,
// 					box_y + i,
// 					&blank);
// 			}
// 		}
// 	}
// }
//
// struct tb_cell* strn_cell(char* s, uint16_t len) // throws
// {
// 	struct tb_cell* cells = malloc((sizeof (struct tb_cell)) * len);
// 	char* s2 = s;
// 	uint32_t c;
//
// 	if (cells != NULL)
// 	{
// 		for (uint16_t i = 0; i < len; ++i)
// 		{
// 			if ((s2 - s) >= len)
// 			{
// 				break;
// 			}
//
// 			s2 += utf8_char_to_unicode(&c, s2);
//
// 			cells[i].ch = c;
// 			cells[i].bg = config.bg;
// 			cells[i].fg = config.fg;
// 		}
// 	}
// 	else
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	return cells;
// }
//
// struct tb_cell* str_cell(char* s) // throws
// {
// 	return strn_cell(s, strlen(s));
// }
//
// void draw_labels(struct term_buf* buf) // throws
// {
// 	// login text
// 	struct tb_cell* login = str_cell(lang.login);
//
// 	if (dgn_catch())
// 	{
// 		dgn_reset();
// 	}
// 	else
// 	{
// 		tb_blit(
// 			buf->box_x + config.margin_box_h,
// 			buf->box_y + config.margin_box_v + 4,
// 			strlen(lang.login),
// 			1,
// 			login);
// 		free(login);
// 	}
//
// 	// password text
// 	struct tb_cell* password = str_cell(lang.password);
//
// 	if (dgn_catch())
// 	{
// 		dgn_reset();
// 	}
// 	else
// 	{
// 		tb_blit(
// 			buf->box_x + config.margin_box_h,
// 			buf->box_y + config.margin_box_v + 6,
// 			strlen(lang.password),
// 			1,
// 			password);
// 		free(password);
// 	}
//
// 	if (buf->info_line != NULL)
// 	{
// 		uint16_t len = strlen(buf->info_line);
// 		struct tb_cell* info_cell = str_cell(buf->info_line);
//
// 		if (dgn_catch())
// 		{
// 			dgn_reset();
// 		}
// 		else
// 		{
// 			tb_blit(
// 				buf->box_x + ((buf->box_width - len) / 2),
// 				buf->box_y + config.margin_box_v,
// 				len,
// 				1,
// 				info_cell);
// 			free(info_cell);
// 		}
// 	}
// }
//
// void draw_f_commands()
// {
// 	struct tb_cell* f1 = str_cell(lang.f1);
//
// 	if (dgn_catch())
// 	{
// 		dgn_reset();
// 	}
// 	else
// 	{
// 		tb_blit(0, 0, strlen(lang.f1), 1, f1);
// 		free(f1);
// 	}
//
// 	struct tb_cell* f2 = str_cell(lang.f2);
//
// 	if (dgn_catch())
// 	{
// 		dgn_reset();
// 	}
// 	else
// 	{
// 		tb_blit(strlen(lang.f1) + 1, 0, strlen(lang.f2), 1, f2);
// 		free(f2);
// 	}
// }
//
// void draw_lock_state(struct term_buf* buf)
// {
// 	// get values
// 	int fd = open(config.console_dev, O_RDONLY);
//
// 	if (fd < 0)
// 	{
// 		buf->info_line = lang.err_console_dev;
// 		return;
// 	}
//
// 	bool numlock_on;
// 	bool capslock_on;
//
// #if defined(__DragonFly__) || defined(__FreeBSD__)
// 	int led;
// 	ioctl(fd, KDGETLED, &led);
// 	numlock_on = led & LED_NUM;
// 	capslock_on = led & LED_CAP;
// #else // linux
// 	char led;
// 	ioctl(fd, KDGKBLED, &led);
// 	numlock_on = led & K_NUMLOCK;
// 	capslock_on = led & K_CAPSLOCK;
// #endif
//
// 	close(fd);
//
// 	// print text
// 	uint16_t pos_x = buf->width - strlen(lang.numlock);
//
// 	if (numlock_on)
// 	{
// 		struct tb_cell* numlock = str_cell(lang.numlock);
//
// 		if (dgn_catch())
// 		{
// 			dgn_reset();
// 		}
// 		else
// 		{
// 			tb_blit(pos_x, 0, strlen(lang.numlock), 1, numlock);
// 			free(numlock);
// 		}
// 	}
//
// 	pos_x -= strlen(lang.capslock) + 1;
//
// 	if (capslock_on)
// 	{
// 		struct tb_cell* capslock = str_cell(lang.capslock);
//
// 		if (dgn_catch())
// 		{
// 			dgn_reset();
// 		}
// 		else
// 		{
// 			tb_blit(pos_x, 0, strlen(lang.capslock), 1, capslock);
// 			free(capslock);
// 		}
// 	}
// }
//
// void draw_desktop(struct desktop* target)
// {
// 	uint16_t len = strlen(target->list[target->cur]);
//
// 	if (len > (target->visible_len - 3))
// 	{
// 		len = target->visible_len - 3;
// 	}
//
// 	tb_change_cell(
// 		target->x,
// 		target->y,
// 		'<',
// 		config.fg,
// 		config.bg);
//
// 	tb_change_cell(
// 		target->x + target->visible_len - 1,
// 		target->y,
// 		'>',
// 		config.fg,
// 		config.bg);
//
// 	for (uint16_t i = 0; i < len; ++ i)
// 	{
// 		tb_change_cell(
// 			target->x + i + 2,
// 			target->y,
// 			target->list[target->cur][i],
// 			config.fg,
// 			config.bg);
// 	}
// }
//
// void draw_input(struct text* input)
// {
// 	uint16_t len = strlen(input->text);
// 	uint16_t visible_len = input->visible_len;
//
// 	if (len > visible_len)
// 	{
// 		len = visible_len;
// 	}
//
// 	struct tb_cell* cells = strn_cell(input->visible_start, len);
//
// 	if (dgn_catch())
// 	{
// 		dgn_reset();
// 	}
// 	else
// 	{
// 		tb_blit(input->x, input->y, len, 1, cells);
// 		free(cells);
//
// 		struct tb_cell c1 = {' ', config.fg, config.bg};
//
// 		for (uint16_t i = input->end - input->visible_start; i < visible_len; ++i)
// 		{
// 			tb_put_cell(
// 				input->x + i,
// 				input->y,
// 				&c1);
// 		}
// 	}
// }
//
// void draw_input_mask(struct text* input)
// {
// 	uint16_t len = strlen(input->text);
// 	uint16_t visible_len = input->visible_len;
//
// 	if (len > visible_len)
// 	{
// 		len = visible_len;
// 	}
//
// 	struct tb_cell c1 = {config.asterisk, config.fg, config.bg};
// 	struct tb_cell c2 = {' ', config.fg, config.bg};
//
// 	for (uint16_t i = 0; i < visible_len; ++i)
// 	{
// 		if (input->visible_start + i < input->end)
// 		{
// 			tb_put_cell(
// 				input->x + i,
// 				input->y,
// 				&c1);
// 		}
// 		else
// 		{
// 			tb_put_cell(
// 				input->x + i,
// 				input->y,
// 				&c2);
// 		}
// 	}
// }
//
// void position_input(
// 	struct term_buf* buf,
// 	struct desktop* desktop,
// 	struct text* login,
// 	struct text* password)
// {
// 	uint16_t x = buf->box_x + config.margin_box_h + buf->labels_max_len + 1;
// 	int32_t len = buf->box_x + buf->box_width - config.margin_box_h - x;
//
// 	if (len < 0)
// 	{
// 		return;
// 	}
//
// 	desktop->x = x;
// 	desktop->y = buf->box_y + config.margin_box_v + 2;
// 	desktop->visible_len = len;
//
// 	login->x = x;
// 	login->y = buf->box_y + config.margin_box_v + 4;
// 	login->visible_len = len;
//
// 	password->x = x;
// 	password->y = buf->box_y + config.margin_box_v + 6;
// 	password->visible_len = len;
// }
//
// static void doom_init(struct term_buf* buf)
// {
// 	buf->init_width = buf->width;
// 	buf->init_height = buf->height;
// 	buf->astate.doom = malloc(sizeof(struct doom_state));
//
// 	if (buf->astate.doom == NULL)
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	uint16_t tmp_len = buf->width * buf->height;
// 	buf->astate.doom->buf = malloc(tmp_len);
// 	tmp_len -= buf->width;
//
// 	if (buf->astate.doom->buf == NULL)
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	memset(buf->astate.doom->buf, 0, tmp_len);
// 	memset(buf->astate.doom->buf + tmp_len, DOOM_STEPS - 1, buf->width);
// }
//
// static void doom_free(struct term_buf* buf)
// {
// 	free(buf->astate.doom->buf);
// 	free(buf->astate.doom);
// }
//
// // Adapted from cmatrix
// static void matrix_init(struct term_buf* buf)
// {
// 	buf->init_width = buf->width;
// 	buf->init_height = buf->height;
// 	buf->astate.matrix = malloc(sizeof(struct matrix_state));
// 	struct matrix_state* s = buf->astate.matrix;
//
// 	if (s == NULL)
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	uint16_t len = buf->height + 1;
// 	s->grid = malloc(sizeof(struct matrix_dot*) * len);
//
// 	if (s->grid == NULL)
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	len = (buf->height + 1) * buf->width;
// 	(s->grid)[0] = malloc(sizeof(struct matrix_dot) * len);
//
// 	if ((s->grid)[0] == NULL)
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	for (int i = 1; i <= buf->height; ++i)
// 	{
// 		s->grid[i] = s->grid[i - 1] + buf->width;
//
// 		if (s->grid[i] == NULL)
// 		{
// 			dgn_throw(DGN_ALLOC);
// 		}
// 	}
//
// 	s->length = malloc(buf->width * sizeof(int));
//
// 	if (s->length == NULL)
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	s->spaces = malloc(buf->width * sizeof(int));
//
// 	if (s->spaces == NULL)
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	s->updates = malloc(buf->width * sizeof(int));
//
// 	if (s->updates == NULL)
// 	{
// 		dgn_throw(DGN_ALLOC);
// 	}
//
// 	// Initialize grid
// 	for (int i = 0; i <= buf->height; ++i)
// 	{
// 		for (int j = 0; j <= buf->width - 1; j += 2)
// 		{
// 			s->grid[i][j].val = -1;
// 		}
// 	}
//
// 	for (int j = 0; j < buf->width; j += 2)
// 	{
// 		s->spaces[j] = (int) rand() % buf->height + 1;
// 		s->length[j] = (int) rand() % (buf->height - 3) + 3;
// 		s->grid[1][j].val = ' ';
// 		s->updates[j] = (int) rand() % 3 + 1;
// 	}
// }
//
// static void matrix_free(struct term_buf* buf)
// {
// 	free(buf->astate.matrix->grid[0]);
// 	free(buf->astate.matrix->grid);
// 	free(buf->astate.matrix->length);
// 	free(buf->astate.matrix->spaces);
// 	free(buf->astate.matrix->updates);
// 	free(buf->astate.matrix);
// }

// void animate_init(struct term_buf* buf)
// {
// 	if (config.animate)
// 	{
// 		switch(config.animation)
// 		{
// 			case 0:
// 			{
// 				doom_init(buf);
// 				break;
// 			}
// 			case 1:
// 			{
// 				matrix_init(buf);
// 				break;
// 			}
// 		}
// 	}
// }
//
// static void doom(struct term_buf* term_buf)
// {
// 	static struct tb_cell fire[DOOM_STEPS] =
// 	{
// 		{' ', 9, 0}, // default
// 		{0x2591, 2, 0}, // red
// 		{0x2592, 2, 0}, // red
// 		{0x2593, 2, 0}, // red
// 		{0x2588, 2, 0}, // red
// 		{0x2591, 4, 2}, // yellow
// 		{0x2592, 4, 2}, // yellow
// 		{0x2593, 4, 2}, // yellow
// 		{0x2588, 4, 2}, // yellow
// 		{0x2591, 8, 4}, // white
// 		{0x2592, 8, 4}, // white
// 		{0x2593, 8, 4}, // white
// 		{0x2588, 8, 4}, // white
// 	};
//
// 	uint16_t src;
// 	uint16_t random;
// 	uint16_t dst;
//
// 	uint16_t w = term_buf->init_width;
// 	uint8_t* tmp = term_buf->astate.doom->buf;
//
// 	if ((term_buf->width != term_buf->init_width) || (term_buf->height != term_buf->init_height))
// 	{
// 		return;
// 	}
//
// 	struct tb_cell* buf = tb_cell_buffer();
//
// 	for (uint16_t x = 0; x < w; ++x)
// 	{
// 		for (uint16_t y = 1; y < term_buf->init_height; ++y)
// 		{
// 			src = y * w + x;
// 			random = ((rand() % 7) & 3);
// 			dst = src - random + 1;
//
// 			if (w > dst)
// 			{
// 				dst = 0;
// 			}
// 			else
// 			{
// 				dst -= w;
// 			}
//
// 			tmp[dst] = tmp[src] - (random & 1);
//
// 			if (tmp[dst] > 12)
// 			{
// 				tmp[dst] = 0;
// 			}
//
// 			buf[dst] = fire[tmp[dst]];
// 			buf[src] = fire[tmp[src]];
// 		}
// 	}
// }

// Adapted from cmatrix
// static void matrix(struct term_buf* buf)
// {
// 	static int frame = 3;
// 	const int frame_delay = 8;
// 	static int count = 0;
// 	bool first_col;
// 	struct matrix_state* s = buf->astate.matrix;
//
// 	// Allowed codepoints
// 	const int randmin = 33;
// 	const int randnum = 123 - randmin;
// 	// Chars change mid-scroll
// 	const bool changes = true;
//
// 	if ((buf->width != buf->init_width) || (buf->height != buf->init_height))
// 	{
// 		return;
// 	}
//
// 	count += 1;
// 	if (count > frame_delay) {
// 		frame += 1;
// 		if (frame > 4) frame = 1;
// 		count = 0;
//
// 		for (int j = 0; j < buf->width; j += 2)
// 		{
// 			int tail;
// 			if (frame > s->updates[j])
// 			{
// 				if (s->grid[0][j].val == -1 && s->grid[1][j].val == ' ')
// 				{
// 					if (s->spaces[j] > 0)
// 					{
// 						s->spaces[j]--;
// 					} else {
// 						s->length[j] = (int) rand() % (buf->height - 3) + 3;
// 						s->grid[0][j].val = (int) rand() % randnum + randmin;
// 						s->spaces[j] = (int) rand() % buf->height + 1;
// 					}
// 				}
//
// 				int i = 0, seg_len = 0;
// 				first_col = 1;
// 				while (i <= buf->height)
// 				{
// 					// Skip over spaces
// 					while (i <= buf->height
// 							&& (s->grid[i][j].val == ' ' || s->grid[i][j].val == -1))
// 					{
// 						i++;
// 					}
//
// 					if (i > buf->height) break;
//
// 					// Find the head of this col
// 					tail = i;
// 					seg_len = 0;
// 					while (i <= buf->height
// 							&& (s->grid[i][j].val != ' ' && s->grid[i][j].val != -1))
// 					{
// 						s->grid[i][j].is_head = false;
// 						if (changes)
// 						{
// 							if (rand() % 8 == 0)
// 								s->grid[i][j].val = (int) rand() % randnum + randmin;
// 						}
// 						i++;
// 						seg_len++;
// 					}
//
// 					// Head's down offscreen
// 					if (i > buf->height)
// 					{
// 						s->grid[tail][j].val = ' ';
// 						continue;
// 					}
//
// 					s->grid[i][j].val = (int) rand() % randnum + randmin;
// 					s->grid[i][j].is_head = true;
//
// 					if (seg_len > s->length[j] || !first_col) {
// 						s->grid[tail][j].val = ' ';
// 						s->grid[0][j].val = -1;
// 					}
// 					first_col = 0;
// 					i++;
// 				}
// 			}
// 		}
// 	}
//
// 	uint32_t blank;
// 	utf8_char_to_unicode(&blank, " ");
//
// 	for (int j = 0; j < buf->width; j += 2) {
// 		for (int i = 1; i <= buf->height; ++i)
// 		{
// 			uint32_t c;
// 			int fg = TB_GREEN;
// 			int bg = TB_DEFAULT;
//
// 			if (s->grid[i][j].val == -1 || s->grid[i][j].val == ' ')
// 			{
// 				tb_change_cell(j, i - 1, blank, fg, bg);
// 				continue;
// 			}
//
// 			char tmp[2];
// 			tmp[0] = s->grid[i][j].val;
// 			tmp[1] = '\0';
// 			if(utf8_char_to_unicode(&c, tmp))
// 			{
// 				if (s->grid[i][j].is_head)
// 				{
// 					fg = TB_WHITE | TB_BOLD;
// 				}
// 				tb_change_cell(j, i - 1, c, fg, bg);
// 			}
// 		}
// 	}
// }

// void animate(struct term_buf* buf)
// {
// 	buf->width = tb_width();
// 	buf->height = tb_height();
//
// 	if (config.animate)
// 	{
// 		switch(config.animation)
// 		{
// 			case 0:
// 			{
// 				doom(buf);
// 				break;
// 			}
// 			case 1:
// 			{
// 				matrix(buf);
// 				break;
// 			}
// 		}
// 	}
// }
//
// bool cascade(struct term_buf* term_buf, uint8_t* fails)
// {
// 	uint16_t width = term_buf->width;
// 	uint16_t height = term_buf->height;
//
// 	struct tb_cell* buf = tb_cell_buffer();
// 	bool changes = false;
// 	char c_under;
// 	char c;
//
// 	for (int i = height - 2; i >= 0; --i)
// 	{
// 		for (int k = 0; k < width; ++k)
// 		{
// 			c = buf[i * width + k].ch;
//
// 			if (isspace(c))
// 			{
// 				continue;
// 			}
//
// 			c_under = buf[(i + 1) * width + k].ch;
//
// 			if (!isspace(c_under))
// 			{
// 				continue;
// 			}
//
// 			if (!changes)
// 			{
// 				changes = true;
// 			}
//
// 			if ((rand() % 10) > 7)
// 			{
// 				continue;
// 			}
//
// 			buf[(i + 1) * width + k] = buf[i * width + k];
// 			buf[i * width + k].ch = ' ';
// 		}
// 	}
//
// 	// stop force-updating
// 	if (!changes)
// 	{
// 		sleep(7);
// 		*fails = 0;
//
// 		return false;
// 	}
//
// 	// force-update
// 	return true;
// }

// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::{io, io::Write, thread, time::Duration};
// use tui::{
//     backend::{Backend, CrosstermBackend},
//     layout::{Constraint, Direction, Layout},
//     widgets::{Block, Borders, Paragraph, Widget},
//     Frame, Terminal,
// };
//
// fn ui<B: Backend>(f: &mut Frame<B>) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(1)
//         .constraints(
//             [
//                 Constraint::Percentage(10),
//                 Constraint::Percentage(80),
//                 Constraint::Percentage(10),
//             ]
//             .as_ref(),
//         )
//         .split(f.size());
//     let block = Block::default().title("Binary").borders(Borders::ALL);
//     f.render_widget(block, chunks[0]);
//     let block = Block::default().title("Input").borders(Borders::ALL);
//     f.render_widget(block, chunks[1]);
//
//     let block = Paragraph::new("Words and text and things that are testing thing. Words and text and things that are testing thing. ").block(
//         Block::default()
//             .title("Block")
//             .borders(Borders::ALL)
//     );
//
//     f.render_widget(block, chunks[2]);
// }
//
//
// fn main() -> Result<(), io::Error> {
//     // setting up the terminal
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//     terminal.clear()?;
//     enable_raw_mode()?;
//     terminal.hide_cursor()?;
//
//     terminal.draw(|f| {
//         ui(f);
//         /*
//         let size = f.size();
//         let block = Block::default()
//             .title("Block")
//             .borders(Borders::ALL);
//         f.render_widget(block, size);
//         */
//     })?;
//
//     thread::sleep(Duration::from_millis(5000));
//
//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;
//
//     /*
//     let mut res: Option<i64> = Option::None;
//     while res.is_none() {
//         println!("Welcome to a this program that convers things");
//         println!("Choose an options:");
//         println!("1. Convert from Binary to Decimal");
//         println!("2. Convert from Decimal to Binary");
//         print!("Enter your choice: "); std::io::stdout().flush().unwrap();
//
//         let mut buffer = String::new();
//         let _ = std::io::stdin().read_line(&mut buffer);
//
//         //checks if the input is a number
//         let num = buffer.trim().parse::<i64>();
//
//         match num {
//             Ok(number) => res = Some(number),
//             Err(_) => {
//                 println!("Invalid choice")
//             }
//         }
//     }
//
//     let num = res.unwrap();
//
//     match num {
//         1 => to_binary(),
//         2 => to_decimal(),
//         _ => {
//             println!("Invalid choice");
//             main();
//         },
//     };
//     */
//
//     Ok(())
// }

// fn to_decimal() {
//     println!("You chose to convert from Decimal to Binary");
//     print!("Enter a decimal number: ");
//     std::io::stdout().flush().unwrap();
//
//     let mut dec_buffer = String::new();
//     let _ = std::io::stdin().read_line(&mut dec_buffer);
//     let mut dec = match dec_buffer.trim().parse::<i128>() {
//         Ok(number) => number,
//         Err(_) => {
//             println!("Do better");
//             to_decimal();
//             return;
//         }
//     };
//
//     let mut bin = String::new();
//     while dec > 0 {
//         if dec % 2 == 0 {
//             bin.push('0');
//         } else {
//             bin.push('1');
//         }
//         dec /= 2;
//     }
//     println!("Out: {}", bin.chars().rev().collect::<String>());
// }
//
// fn to_binary() {
//     println!("You chose to convert from Binary to Decimal");
//     print!("Enter a binary number: ");
//     std::io::stdout().flush().unwrap();
//
//     let mut bin_buffer = String::new();
//     let _ = std::io::stdin().read_line(&mut bin_buffer);
//     let bin: &str = bin_buffer.trim();
//
//     let mut dec: i128 = 0;
//     let mut count = 0;
//
//     for c in bin.chars().rev() {
//         match c {
//             '0' => (),
//             '1' => dec += 2_i128.pow(count),
//             _ => {
//                 println!("Invalid binary number, try again");
//                 to_binary();
//                 return;
//             }
//         }
//         count += 1;
//     }
//     println!("Out: {}", dec);
// }
//
// struct MatrixDot {
// 	val: i32,
// 	is_head: bool,
// }

// struct MatrixState {
// 	grid: Vec<Vec<MatrixDot>>,
// 	updates: i32,
// }

// fn draw_init(struct term_buf* buf);
// void draw_free(struct term_buf* buf);
// void draw_box(struct term_buf* buf);

// struct tb_cell* strn_cell(char* s, uint16_t len);
// struct tb_cell* str_cell(char* s);

// void draw_input(struct text* input);
// void draw_input_mask(struct text* input);

// void position_input(
// 	struct term_buf* buf,
// 	struct desktop* desktop,
// 	struct text* login,
// 	struct text* password);
//
// void animate_init(struct term_buf* buf);
// void animate(struct term_buf* buf);
// bool cascade(struct term_buf* buf, uint8_t* fails);

// #define DOOM_STEPS 13
