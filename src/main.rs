mod cli;
mod color;
mod hint;
mod screen;
mod settings;
mod utils;

use std::env;

use std::io;
use std::io::Read;

use screen::Screen;
use settings::Settings;
use std::io::Write;

#[macro_use]
extern crate clap;

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
                    utils::swap_pane();
                    return;
                }
            }
            // paste in console
            'p' => {
                utils::swap_pane();
                utils::tmux_run(&["send", screen.selected()]);
                return;
            }
            // exit
            'q' => {
                utils::swap_pane();
                return;
            },
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

    // Avoid flickering by moving here
    utils::swap_pane();

    read_loop(&mut screen);
}

/// Entrypoint, when there's no arguments it starts an inner window in tmux
/// calling itself with inner argument
fn main() {
    let args = cli::args();
    Settings::init();

    match args.subcommand_name() {
        Some("inner") => inner(),
        Some("config") => println!("{}", Settings::serialize()),
        // Should not happend
        Some(subcmd) => println!("Invalid subcmand {}", subcmd),

        // Get current binary and re-run with innner command in a new window
        None => {
            let binary = env::args().nth(0).unwrap();
            utils::open_inner_window("Hint Select", &binary);
        }
    };
}
