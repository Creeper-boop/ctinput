use std::process::Command;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Lookup {
    pub key: [u8; 2],
    pub handle: JoinHandle<()>,
}

impl Lookup {
    pub fn new(key: [u8; 2], handle: JoinHandle<()>) -> Lookup {
        Lookup {
            key,
            handle,
        }
    }
}

#[derive(Clone)]
pub struct Runner {
    active_tui: String,
    inactive_tui: String,
    command: String,
    args: Vec<String>,
}

impl Runner {
    pub fn new(active_tui: String, inactive_tui: String, command: String, args: Vec<String>) -> Runner {
        Runner {
            active_tui,
            inactive_tui,
            command,
            args,
        }
    }

    pub fn run(&mut self) -> JoinHandle<()> {
        let data = self.clone();
        thread::spawn(move || {
            print!("{}", data.active_tui);
            let _ = Command::new(&data.command).args(&data.args).status().unwrap();
            print!("{}", data.inactive_tui);
        })
    }
}
