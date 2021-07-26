use libc::setsid;
use regex::Regex;
use std::ffi::OsStr;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use crate::settings::Settings;

const INNER_WINDOW: &str = "999999";
const INNER_PANE: &str = "999999.0";

pub fn tmux<I, S>(args: I) -> Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("tmux");
    command.args(args);
    command
}

pub fn tmux_run<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    tmux(args)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Can't run command")
        .wait()
        .expect("Can't run command");
}

pub fn tmux_output<I, S>(args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let out = tmux(args).output().expect("Can't run command");
    String::from_utf8_lossy(&out.stdout).to_string()
}

pub fn open_url(url: &str) {
    let settings = Settings::global();

    // xdg-open opens urls without http as a file
    let url = if url.starts_with("www.") {
        format!("http://{}", url)
    } else {
        url.into()
    };

    Command::new(&settings.opener)
        .before_exec(|| unsafe {
            setsid();
            Ok(())
        })
        .arg(url)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Can't open url");
}

pub fn open_inner_window(_title: &str, command: &str) {
    let (height, width) = pane_size().unwrap();
    let command = format!("{} inner", command);
    tmux_run(&["new-window", "-dn", "", "-t", INNER_WINDOW, &command]);
    // Remove status format in new window
    tmux_run(&["setw", "-qt", INNER_WINDOW, "window-status-format", ""]);
    tmux_run(&["setw", "-qt", INNER_WINDOW, "force-height", &height.to_string()]);
    tmux_run(&["setw", "-qt", INNER_WINDOW, "force-width", &width.to_string()]);
    tmux_run(&["setw", "-qt", INNER_WINDOW, "window-status-current-format", ""]);
}

pub fn display(msg: &str) {
    tmux_run(&["display-message", &format!("[tmux-hints] {}", msg)]);
}

pub fn capture_pane() {
    tmux_run(&["capture-pane", "-eJb", "tmux-hints-buffer"]);
}

pub fn get_buffer() -> String {
    tmux_output(&["show-buffer", "-b", "tmux-hints-buffer"])
        .trim()
        .to_string()
}

pub fn clear_buffer() {
    tmux_run(&["delete-buffer", "-b", "tmux-hints-buffer"]);
}

pub fn clean_string(buffer: &str) -> String {
    let rectrl = Regex::new(r"\x1b[^m]*m|\p{Format}").unwrap();
    rectrl.replace_all(&buffer, "").to_string()
}

pub fn pane_size() -> Option<(String, String)> {
    let output = tmux_output(&["list-panes", "-F", "#{pane_active},#{pane_height},#{pane_width}"]);
    let panes: Vec<&str> = output.trim().split('\n').collect();
    for pane_str in &panes {
        let pane: Vec<&str> = pane_str.split(',').collect();
        if pane[0] == "1" {
            return Some((pane[1].to_owned(), pane[2].to_owned()));
        }
    };

    None
}

pub fn swap_pane() {
    tmux_run(&["swap-pane", "-t", INNER_PANE]);
}

pub fn cursor_at(x: usize, y: usize) -> String {
    format!("\x1B[{};{}H", y + 1, x + 1)
}
