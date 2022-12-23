extern crate core;

use std::sync::mpsc;
use std::{env, io, thread};
use std::io::{Read, Write};
use std::process::id;
use std::time::Duration;
use libc::pid_t;
use nix::unistd::{Pid, Uid};

use signal_hook::{consts::signal::*, iterator::Signals};

use crate::input::InputToolset;
use crate::tui::ScriptedTui;

mod input;
mod tui;
mod history;
mod apm;
mod runner;

fn main() {
    if Uid::effective().is_root() { panic!("This binary shouldn't be run with root permissions!") }
    // argument variables
    let mut enable_debug = false;
    let mut enable_log = false;
    let mut enable_raw = true;
    let mut enable_runners = false;
    let mut path = "Tui";
    // check for all arguments
    let args: Vec<String> = env::args().collect();
    args.iter().enumerate().for_each(|(i, arg)| match arg.as_str() {
        "-d" | "--debug" => enable_debug = true,
        "-l" | "--log" => enable_log = true,
        "-c" | "--compat" => enable_raw = false,
        "-r" | "--runners" => enable_runners = true,
        "-p" | "--path" => path = &args[i + 1],
        _ => {}
    });
    // initialisation for raw mode
    let mut old_ter = tui::get_dummy_attributes();
    let (input_tx, input_rx) = mpsc::channel();
    if enable_raw { // sets terminal into raw mode and spawns thread to read inputs
        old_ter = tui::set_raw_mode();
        thread::spawn(move || loop {
            let mut buffer = [0u8; 1];
            io::stdin().lock().read_exact(&mut buffer).unwrap();
            input_tx.send(buffer).unwrap();
        });
    }
    // listener initialisation
    let (key_tx, key_rx) = mpsc::channel();
    let mut input_toolset = InputToolset::new(key_tx, enable_log);
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
    let mut debug_key = [0u8, 0u8];
    let mut debug_input = [0u8];
    let (mut height, mut width) = tui::get_size();
    let (static_element, reactive_elements, mut scripted_elements, runners) = tui::load(path);

    // main loop start
    print!("\x1b[H\x1b[J\x1b[?25l");
    print!("{}", static_element);
    loop {
        // system signals handling
        for signal in signals.pending() {
            match signal {
                SIGWINCH => {
                    (height, width) = tui::get_size();
                    print!("{}", static_element);
                } // size updates
                SIGTERM | SIGINT | SIGQUIT | SIGHUP => {
                    // terminal reset and process exit
                    if enable_raw { tui::set_mode(old_ter); }
                    print!("\x1b[0m\x1b[H\x1b[J\x1b[?25h");
                    let _ = io::stdout().lock().flush();
                    std::process::exit(0);
                }
                _ => unreachable!()
            }
        };
        // key detection handling
        while let Some(key) = key_rx.recv_timeout(Duration::from_millis(10)).ok() {
            debug_key = key;
            if let Some(action) = reactive_elements.get(key.as_ref()) { print!("{}", action); }
            // scripted element handling
            scripted_elements.iter_mut().for_each(|e| match e {
                ScriptedTui::History(history) => history.add_event(key[1], key[0]),
                ScriptedTui::Apm(apm) => if key[0] == 13u8 || key[0] == 15u8 { apm.tick() },
            });
            // runner handling
            if enable_runners { if let Some(runner) = runners.get(key.as_ref()) { runner::run(runner) } }

            match key[0] {
                13u8 | 15u8 => {
                    keys.push(key[1]);
                } // 13 represents pressed keys 15 represents pressed mouse buttons
                14u8 | 16u8 => { // 14 represents released keys 16 represents released mouse buttons
                    if let Some(i) = keys.iter().position(|&r| r == key[1]) {
                        keys.remove(i); // better code used to mitigate crashes if a key is logged twice
                    }
                }
                _ => {} // just pass if no matches
            }
        }
        // raw input handling
        if enable_raw {
            while let Some(input) = input_rx.recv_timeout(Duration::from_millis(10)).ok() {
                debug_input = input;
                match input {
                    // [3] being the ctrl + c code the usual signal is then sent
                    [3] => { nix::sys::signal::kill(Pid::from_raw(id() as pid_t), nix::sys::signal::SIGINT).unwrap(); }
                    // [12] being the ctrl + l code for clear signals for reload witch should happen when rescaled
                    [12] => { nix::sys::signal::kill(Pid::from_raw(id() as pid_t), nix::sys::signal::SIGWINCH).unwrap(); }
                    // just pass if no matches
                    _ => {}
                }
            }
        }
        // scripted element tui
        scripted_elements.iter().for_each(|e| match e {
            ScriptedTui::History(history) => print!("{}", history.get_tui(width)),
            ScriptedTui::Apm(apm) => print!("{}", apm.get_tui(width)),
        });

        // debug data
        if enable_debug { print!("\x1b[{}H\x1b[Kw:{} h:{} {:?} [g{:?} l{:?}]", height, width, height, keys, debug_key, debug_input); }

        let _ = io::stdout().lock().flush();
    }
}
