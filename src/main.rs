extern crate core;

use std::sync::mpsc;
use std::{env, io, thread};
use std::io::{Read, Write};
use std::process::id;
use std::time::{Duration, SystemTime};
use libc::pid_t;
use nix::unistd::Pid;

use signal_hook::{consts::signal::*, iterator::Signals};

use crate::input::InputToolset;

mod input;
mod tui;

fn main() {
    // argument variables
    let mut debug = false;
    let mut log = false;
    let mut raw = true;
    // check for all arguments
    env::args().for_each(|arg| match &*arg {
        "-d" | "--debug" => debug = true,
        "-l" | "--log" => log = true,
        "-c" | "--compat" => raw = false,
        _ => {}
    });
    // initialisation for raw mode
    let mut old_ter = tui::get_dummy_attributes();
    let (input_tx, input_rx) = mpsc::channel();
    if raw { // sets terminal into raw mode and spawns thread to read inputs
        old_ter = tui::set_raw_mode();
        thread::spawn(move || loop {
            let mut buffer = [0u8; 1];
            io::stdin().lock().read_exact(&mut buffer).unwrap();
            input_tx.send(buffer).unwrap();
        });
    }
    // listener initialisation
    let (key_tx, key_rx) = mpsc::channel();
    let mut input_toolset = InputToolset::new(key_tx, log);
    thread::spawn(move || { input_toolset.thread() });
    // system signal setup
    let mut signals = Signals::new(&[
        SIGWINCH,
        SIGTERM,
        SIGINT,
        SIGQUIT,
        SIGHUP
    ]).unwrap();
    // global variable allocation
    let mut keys: Vec<u8> = Vec::new();
    let (mut width, mut height) = tui::get_size();
    // main loop start
    println!("\x1b[H\x1b[J\x1b[?25l");
    loop {
        // system signals handling
        for signal in signals.pending() {
            match signal {
            SIGWINCH => (width, height) = tui::get_size(),
            SIGTERM | SIGINT | SIGQUIT | SIGHUP => {
                if raw {tui::set_mode(old_ter);}
                println!("\x1b[0m\x1b[H\x1b[J\x1b[?25h");
                std::process::exit(0);
            },
            _ => unreachable!()
        }};
        // key detection handling
        let key = key_rx.recv_timeout(Duration::from_millis(50)).unwrap_or([0u8, 0u8]);
        if key[0] == 13u8 || key[0] == 15u8 { keys.push(key[1]) }
        if key[0] == 14u8 || key[0] == 16u8 {
            if let Some(i) = keys.iter().position(|&r| r == key[1]) {
                keys.remove(i);
            }
        }
        if debug { print!("\x1b[{}H\x1b[Kw:{} h:{} {keys:?} {} {}", height - 1, width, height, key[0], key[1]); } // debug pressed keys
        // raw input handling
        if raw {
            let input = input_rx.recv_timeout(Duration::from_millis(50)).unwrap_or([0u8]);
            if debug {print!(" {:?}", input);} // debug inputted keys
            if input == [3] { nix::sys::signal::kill(Pid::from_raw(id() as pid_t), nix::sys::signal::SIGINT).unwrap();}
        }

        let _ = io::stdout().lock().flush();
    }
}
