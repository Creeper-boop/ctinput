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
pub fn load_from_file(path: &str) -> String {
    let mut file = String::new();
    BufReader::new(File::open(path).unwrap()).read_line(&mut file).expect("Invalid tui file path!");
    file.replace(r#"\x1b["#, "\x1b[")// the first line should always represent the background
}
