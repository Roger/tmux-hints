use libc::setsid;
use regex::Regex;
use std::ffi::OsStr;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use crate::settings::Settings;

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
    let command = format!("{} inner", command);
    tmux_run(&["new-window", "-dn", "", "-t", "999999", &command]);
    // Remove status format in new window
    tmux_run(&["setw", "-qt", "999999", "window-status-format", ""]);
    tmux_run(&["setw", "-qt", "999999", "window-status-current-format", ""]);
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
    let rectrl = Regex::new(r"\x1b[^m]*m").unwrap();
    let reunicode = Regex::new(r"[^\x00-\x7F]").unwrap();

    reunicode
        .replace_all(&rectrl.replace_all(buffer, ""), " ")
        .to_string()
}

pub fn select_window(title: &str) {
    tmux_run(&["select-window", "-t", title]);
}

pub fn select_last() {
    tmux_run(&["last-window"]);
}

pub fn get_terminal_size() -> (usize, usize) {
    let out = Command::new("stty")
        .arg("-F")
        .arg("/dev/tty")
        .arg("size")
        .output()
        .expect("Can't run stty");

    let result = String::from_utf8_lossy(&out.stdout).to_string();
    let result: Vec<&str> = result.trim().split(' ').collect();

    match result.as_slice() {
        [a, b] => (a.parse().unwrap(), b.parse().unwrap()),
        _ => panic!("stty invalid output"),
    }
}

pub fn cursor_at(x: usize, y: usize) -> String {
    format!("\x1B[{};{}H", y + 1, x + 1)
}
