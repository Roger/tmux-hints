use hint::Hint;
use utils;

use std::io;
use std::os::unix::io::AsRawFd;
use regex::Regex;

use termios::*;

/// Screen represents the Pane of tmux
#[derive(Debug)]
pub struct Screen {
    buffer: String,
    selected: usize,
    size: (usize, usize),
    hints: Vec<Hint>,
}

impl Screen {
    /// Returns a screen(tmux pane)
    ///
    /// # Example
    ///
    /// ```
    /// use screen::Screen;
    /// let screen = Screen::new();
    /// ```

    pub fn new() -> Screen {
        utils::capture_pane();
        let buffer = utils::get_buffer();
        utils::clear_buffer();
        Screen {
            buffer,
            selected: 0,
            size: utils::get_terminal_size(),
            hints: vec![],
        }
    }

    pub fn init_screen(&self) {
        let stdin_fd = io::stdin().as_raw_fd();

        // no echo and canonical mode
        let mut termios = Termios::from_fd(stdin_fd).unwrap();
        termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(stdin_fd, TCSANOW, &termios).unwrap();
        // Hide Cursor
        print!("\x1B[?25l");
    }

    pub fn paint(&self) {
        print!("{}", self.buffer.trim());
        for hint in &self.hints {
            hint.unselect();
        }
        self.hints[0].select();

        // Avoid flickering by moving here
        utils::select_window("999999");
    }

    pub fn prev(&mut self) {
        let selected = if self.selected == self.hints.len() - 1 {
            0
        } else {
            self.selected + 1
        };
        self.select(selected).unwrap();
    }

    pub fn next(&mut self) {
        let selected = match self.selected {
            0 => self.hints.len() - 1,
            _ => self.selected - 1,
        };
        self.select(selected).unwrap();
    }

    pub fn select(&mut self, number: usize) -> Result<(), String> {
        if self.hints.len() <= number {
            return Err(String::from("Hint not found"));
        };

        self.hints[self.selected].unselect();
        self.selected = number;
        self.hints[self.selected].select();

        Ok(())
    }

    pub fn selected(&self) -> &str {
        &self.hints[self.selected].text
    }

    pub fn find_hints(&mut self) -> Result<(), ()> {
        // regex stolen from urxvtperls url-select.pl
        let re_str = r"(?:https?://|ftp://|git://|mailto:|file://|www\.)[\w\-@;/?:&=%\$_.+!*\x27(),~#]+[\w\-@;/?&=%\$_+!*\x27~]";
        let matcher = Regex::new(re_str).unwrap();

        let clean_buffer = utils::clean_string(&self.buffer);
        self.hints = vec![];

        let (_heigth, width) = self.size;

        let mut offset = 0;
        for (i, line) in clean_buffer.lines().enumerate() {
            let y = i + offset;
            // ignore empty lines
            if line.is_empty() {
                continue;
            }
            offset += (line.len()-1) / width;
            for ma in matcher.find_iter(line) {
                let start = ma.start();
                let end = ma.end();

                let text = line[start..end].to_string();

                let start_in_line = start / width;

                let screen_x = start - width * start_in_line;
                let screen_y = y + start_in_line;

                self.hints.push(Hint::new(text, screen_x, screen_y));
            }
        }

        self.hints.reverse();
        for (index, hint) in self.hints.iter_mut().enumerate() {
            hint.set_prefix(index.to_string());
        }

        if self.hints.len() == 0 {
            Err(())
        } else {
            Ok(())
        }
    }
}
