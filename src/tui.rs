use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use libc::ioctl;
use libc::TIOCGWINSZ;
use nix::pty::Winsize;
use nix::sys::termios;
use nix::sys::termios::Termios;

/*
    TUI coding and explanation can be found here:
    https://poor.dev/terminal-anatomy/
 */

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

    unsafe {ioctl(1, TIOCGWINSZ, &mut winsize)};
    (winsize.ws_row, winsize.ws_col)
}
// load tui from file todo implement reactive element loading and reactive elements in general
pub fn load(path: &str) -> (String, HashMap<[u8; 2], String>) {
    let mut file = BufReader::new(File::open(path).unwrap());
    let mut static_tui = String::new();
    let mut reactive_tui = HashMap::new();

    file.read_line(&mut static_tui).expect("Invalid tui file path!");

    let mut line = String::new();
    while file.read_line(&mut line).ok().unwrap() != 0 {
        if line.trim().is_empty() {break}
        let (input, action) = line.split_once("]").unwrap();
        reactive_tui.insert(
            input.trim_start_matches("[").split(",").map(|e| e.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>().try_into().unwrap(),
            action.replace(r#"\x1b["#, "\x1b[")
        );
        line.clear();
    }
    // file documentation is included in the ExampleTui
    (static_tui.replace(r#"\x1b["#, "\x1b["), reactive_tui)
}
