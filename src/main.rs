use std::sync::mpsc;
use std::{env, thread};
use std::time::{Duration, SystemTime};

use crate::gui::Graphics;
use crate::input::InputToolset;

mod input;
mod gui;

fn main() {
    // args 1 being the first argument 0 represents running the binary
    let arg: bool = match env::args().nth(1).unwrap_or("".to_string()).as_str() {
        "true" => true,
        _ => false
    };
    let (tx, rx) = mpsc::channel();
    let mut input_toolset = InputToolset::new(tx, arg);
    // spawn global listener thread
    thread::spawn(move || { input_toolset.thread() });
    // gui
    let mut graphics = Graphics::new();
    graphics.add(1, 2, 6, 3);
    graphics.add(9, 2, 5, 3);
    graphics.add(13, 2, 5, 3);
    graphics.add(17, 2, 5, 3);
    graphics.add(6, 4, 5, 3);
    graphics.add(10, 4, 5, 3);
    graphics.add(14, 4, 5, 3);
    graphics.add(0, 6, 9, 3);
    graphics.add(0, 8, 8, 3);
    graphics.add(13, 8, 9, 3);
    let layer = graphics.generate(true);
    // allocate variables for "ui"
    let mut keys: Vec<u8> = Vec::new();

    let mut copter = 0u128;
    let mut e = 0u128;
    let time = SystemTime::now();
    loop { // shell "ui"
        let recv = rx.recv_timeout(Duration::from_millis(100)).unwrap_or([0u8, 0u8]);
        if recv[0] == 13u8 || recv[0] == 15u8 { keys.push(recv[1]) }
        if recv[0] == 14u8 || recv[0] == 16u8 {
            if let Some(i) = keys.iter().position(|&r| r == recv[1]) {
                keys.remove(i);
            }
        }
        print!("\x1b[2J\x1b[0;0H\x1b[0m");
        println!("{:?} | {} {}", keys, recv[0], recv[1]);

        println!("{}", layer);
        keys.iter().for_each(|k|
            match k {
                1u8 => println!("\x1b[3;3H\x1b[48;2;128;128;128m L\x1b[0m"), // mouse left
                3u8 => println!("\x1b[3;5H\x1b[48;2;128;128;128mR \x1b[0m"), // mouse right
                25u8 => println!("\x1b[3;11H\x1b[48;2;128;128;128m W \x1b[0m"), // w
                26u8 => println!("\x1b[3;15H\x1b[48;2;128;128;128m E \x1b[0m"), // e
                27u8 => println!("\x1b[3;19H\x1b[48;2;128;128;128m R \x1b[0m"), // r
                37u8 => println!("\x1b[9;2H\x1b[48;2;128;128;128m CTRL \x1b[0m"), // ctrl
                38u8 => println!("\x1b[5;8H\x1b[48;2;128;128;128m A \x1b[0m"), // a
                39u8 => println!("\x1b[5;12H\x1b[48;2;128;128;128m S \x1b[0m"), // s
                40u8 => println!("\x1b[5;16H\x1b[48;2;128;128;128m D \x1b[0m"), // d
                50u8 => println!("\x1b[7;2H\x1b[48;2;128;128;128m SHIFT \x1b[0m"), // shift
                65u8 => println!("\x1b[9;15H\x1b[48;2;128;128;128m SPACE \x1b[0m"), // space
                _ => ()
            });
        // copter bad KEK if becomes annoying it did its job :)
        if keys.contains(&26u8) { e = time.elapsed().unwrap().as_millis() }
        if keys.contains(&50u8) && (time.elapsed().unwrap().as_millis() < e + 250) {
            copter = time.elapsed().unwrap().as_millis();
        }
        if time.elapsed().unwrap().as_millis() < copter + 1000 && copter != 0 {
            println!("\x1b[7;20H\x1b[48;2;255;0;0m  \x1b[0m");
        }
        if time.elapsed().unwrap().as_millis() < copter + 5000 && copter != 0 {
            println!("\x1b[7;11H\x1b[48;2;128;0;0m COPTER \x1b[0m");
        }

        println!("\x1b[0;0H");
    }
}
