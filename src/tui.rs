use libc::ioctl;
use libc::TIOCGWINSZ;
use nix::pty::Winsize;
use nix::sys::termios;
use nix::sys::termios::Termios;

/* ANSI for effect
    "\x1b["         being the escape char
    "2J"            clear
    "y;xH"          move the cursor to x, y
    "38;2;r;g;bm"   foreground color to r, g, b
    "48;2;r;g;bm"   background color to r, g, b
    "0m"            reset
    "nm"            setting the mode to n, being from 1..9

    TUI coding and explanation can be found here:
    https://poor.dev/terminal-anatomy/
 */
pub fn get_dummy_attributes() -> Termios {
    // TODO somehow make dummy struct to make program usable completely without raw
    termios::tcgetattr(0).expect("unable to get terminal attribute")
}

pub fn set_raw_mode() -> Termios {
    let mut tio = termios::tcgetattr(0).expect("unable to get terminal attribute");
    let old = tio.clone();
    termios::cfmakeraw(&mut tio);
    match termios::tcsetattr(0, termios::SetArg::TCSANOW, &tio) {
        Ok(_) => {}
        Err(e) => panic!("err {e:?}"),
    };
    old
}

pub fn set_mode(attr: Termios) {
    match termios::tcsetattr(0, termios::SetArg::TCSANOW, &attr) {
        Ok(_) => {}
        Err(e) => panic!("err {e:?}"),
    };
}

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