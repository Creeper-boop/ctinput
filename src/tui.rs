use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use libc::ioctl;
use libc::TIOCGWINSZ;
use nix::pty::Winsize;
use nix::sys::termios;
use nix::sys::termios::Termios;

use crate::history::History;
use crate::apm::Apm;
use crate::runner::Runner;

/*
    TUI coding and explanation can be found here:
    https://poor.dev/terminal-anatomy/
    https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
 */

pub enum ScriptedTui {
    History(History),
    Apm(Apm),
}

// get dummy Termios instance
pub fn get_dummy_attributes() -> Termios {
    // TODO somehow make dummy struct to make program usable completely without raw
    termios::tcgetattr(0).expect("unable to get terminal attribute")
}

// enable raw input mode and return previous state
pub fn set_raw_mode() -> Termios {
    let mut tio = termios::tcgetattr(0).expect("unable to get terminal attribute");
    let old = tio.clone();
    termios::cfmakeraw(&mut tio);
    match termios::tcsetattr(0, termios::SetArg::TCSANOW, &tio) {
        Ok(_) => {}
        Err(e) => panic!("err {:?}", e),
    };
    old
}

// set mode of terminal emulator
pub fn set_mode(attr: Termios) {
    match termios::tcsetattr(0, termios::SetArg::TCSANOW, &attr) {
        Ok(_) => {}
        Err(e) => panic!("err {:?}", e),
    };
}

// get current terminal emulator size
pub fn get_size() -> (u16, u16) {
    let mut winsize = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    unsafe { ioctl(1, TIOCGWINSZ, &mut winsize) };
    (winsize.ws_row, winsize.ws_col)
}

// load tui from file
pub fn load(path: &str) -> (String, HashMap<[u8; 2], String>, Vec<ScriptedTui>, HashMap<[u8; 2], Runner>) {
    let mut file = BufReader::new(File::open(path).unwrap());
    let mut static_tui = String::new();
    let mut reactive_tui = HashMap::new();
    let mut scripted_tui: Vec<ScriptedTui> = Vec::new();
    let mut runners = HashMap::new();


    file.read_line(&mut static_tui).expect("Invalid tui file path!");

    let mut line = String::new();
    while file.read_line(&mut line).ok().unwrap() != 0 {
        if line.trim().is_empty() { break; }
        // if the line represents a reactive element
        if line.starts_with("[") {
            let (input, action) = line.split_once("]").unwrap();
            reactive_tui.insert(
                input.trim_start_matches("[").split(",").map(|e| e.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>().try_into().unwrap(),
                action.replace(r#"\x1b["#, "\x1b["),
            );
        } else if line.starts_with("history") {
            let attributes: Vec<u8> = line.split_whitespace().skip(1).map(|e| e.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>().try_into().unwrap();
            if attributes.len() == 4 {
                scripted_tui.push(ScriptedTui::History(History::new(attributes[0], attributes[1], attributes[2], attributes[3])))
            }
        } else if line.starts_with("apm") {
            let attributes: Vec<u8> = line.split_whitespace().skip(1).map(|e| e.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>().try_into().unwrap();
            if attributes.len() == 3 {
                scripted_tui.push(ScriptedTui::Apm(Apm::new(attributes[0], attributes[1], attributes[2])))
            }
        } else if line.starts_with("runner") {
            let (key, data) = line.trim_start_matches("runner").split_once(" ACTIVE:").unwrap();
            let (active, data) = data.split_once(" INACTIVE:").unwrap();
            let (inactive, data) = data.split_once(" COMMAND:").unwrap();
            let mut data = data.split_whitespace().map(|e| e.to_string()).collect::<Vec<String>>();
            runners.insert(
                key.trim().trim_start_matches("[").trim_end_matches("]").split(",").map(|e| e.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>().try_into().unwrap(),
                Runner::new(active.replace(r#"\x1b["#, "\x1b["), inactive.replace(r#"\x1b["#, "\x1b["), data.remove(0), data),
            );
        }
        line.clear();
    }

    // file documentation is included in the ExampleTui
    (String::from(static_tui.replace(r#"\x1b["#, "\x1b[").trim()), reactive_tui, scripted_tui, runners)
}
