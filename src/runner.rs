use std::process::Command;
use std::thread;

#[derive(Clone)]
pub struct Runner {
    active: String,
    inactive: String,
    command: String,
    args: Vec<String>,
}

impl Runner {
    pub fn new(active: String, inactive: String, command: String, args: Vec<String>) -> Runner {
        Runner {
            active,
            inactive,
            command,
            args,
        }
    }
}

pub fn run(runner: &Runner) {
    let runner = runner.clone();
    thread::spawn(move || {
        print!("{}", runner.active);
        let _ = Command::new(runner.command).args(runner.args).status().unwrap();
        print!("{}", runner.inactive);
    });
}
