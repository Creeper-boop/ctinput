use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdout, Command, Stdio};
use std::sync::mpsc::Sender;
use std::time::SystemTime;

pub struct InputToolset {
    reader: BufReader<ChildStdout>,
    file: File,
    log: bool,
    time: SystemTime,
    tx: Sender<[u8; 2]>,
}

impl InputToolset {
    pub fn new(tx: Sender<[u8; 2]>, log: bool) -> InputToolset {
        // create what is effectively an Xserver logger
        // run xinput command as subprocess to log global keys
        let child = Command::new("xinput").arg("test-xi2").arg("--root")
            .stdout(Stdio::piped()).spawn().expect("Failed to capture input!").stdout.take().unwrap();
        let reader = BufReader::new(child);
        // logging file creation
        let file = File::create("Input.log").unwrap();
        let time = SystemTime::now();

        InputToolset {
            reader,
            file,
            log,
            time,
            tx,
        }
    }

    pub fn thread(&mut self) {
        let mut output = String::new();
        let mut key: [u8; 2] = [0, 0];
        let mut trim = false;
        loop {
            let mut line = String::new();
            self.reader.read_line(&mut line).unwrap(); // receive line from command
            let tmp = line.split_whitespace().find(|i| i.parse::<u8>().is_ok());
            if line.contains("EVENT") { // meaning the next lines will tell what kind of event
                if self.log || !trim { // write the previous line to the output file
                    writeln!(self.file, "{}", output).unwrap();
                } // multiple received lines are compacted into one output line
                trim = true; // lines include redundant data
                output.clear(); // clear last line and log event time and type
                output += &(self.time.elapsed().unwrap().as_millis().to_string() + "|" + tmp.unwrap());
                // key is used for thread communication its first part being the event type
                key[0] = output.clone().split("|").collect::<Vec<&str>>()[1].parse::<u8>().unwrap();
                // different lines require different handling the device line includes only one number
            } else if line.contains("device:") { output += tmp.unwrap(); } else {
                // all other lines can be handled by simple addition to file
                if !&trim { output += &*line } else { output += line.trim() }
                // if !trim received lines represent user input device data
                // output to main thread required only if specific events happened
                // 13 and 14 representing key presses and releases respectively
                // 15 and 16 represent scroll wheel events
                if key[0] == 13u8 || key[0] == 14u8 || key[0] == 15u8 || key[0] == 16u8 {
                    if line.contains("detail:") { // this line includes key codes
                        key[1] = tmp.unwrap().parse::<u8>().unwrap();
                        self.tx.send(key).unwrap(); // being required for key showcase
                    }
                }
            }
            if trim { output += "|" } // add parsing for later use of the log file
            line.clear();
        }
    }
}
