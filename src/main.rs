extern crate dirs;
extern crate libc;
extern crate regex;
extern crate termios;

mod color;
mod hint;
mod screen;
mod utils;

use std::env;

use std::io;
use std::io::Read;

use screen::Screen;
use std::io::Write;

/// Read input loop
fn read_loop(screen: &mut Screen) {
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    loop {
        let mut buffer = [0; 1]; // read exactly one byte
        stdin_handle
            .read_exact(&mut buffer)
            .expect("Can't read stdin");

        let key = buffer[0] as char;
        match key {
            // movement
            'j' => screen.next(),
            'k' => screen.prev(),
            '0'...'9' => {
                if let Err(error) = screen.select(key.to_digit(10).unwrap() as usize) {
                    utils::display(&error);
                }
            }
            // open
            'o' | 'O' => {
                let selected = screen.selected();
                utils::open_url(selected);
                utils::display(&format!("Opening: {}", selected));
                if key != 'O' {
                    utils::select_last();
                    return;
                }
            }
            // paste in console
            'p' => {
                utils::select_last();
                utils::tmux_run(&["send", screen.selected()]);
                return;
            }
            // exit
            'q' => return,
            _ => utils::display(&format!("Unknown key: {}", key)),
        }

        io::stdout().flush().unwrap();
    }
}

/// This is the main function of our program that open in an inner window in tmux
fn inner() {
    let mut screen = Screen::new();
    if screen.find_hints().is_err() {
        utils::display("No Hints found");
        return;
    }

    screen.init_screen();
    screen.paint();
    io::stdout().flush().unwrap();

    read_loop(&mut screen);
}

/// Entrypoint, when there's no arguments it starts an inner window in tmux
/// calling itself with inner argument
fn main() {
    let mut args = env::args();

    // Run itself in the new window
    if args.len() == 1 {
        let arg = args.nth(0).unwrap();
        utils::open_inner_window("Hint Select", &arg);

    // Capture the output and move to our window
    } else if args.nth(1).unwrap() == "inner" {
        inner();
    } else {
        println!("Invalid commandline");
    }
}
