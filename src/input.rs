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
        let child = Command::new("xinput").arg("test-xi2").arg("--root")
            .stdout(Stdio::piped()).spawn().expect("Failed to capture input!").stdout.take().unwrap();
        let reader = BufReader::new(child);

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
        let mut o = String::new();
        let mut k: [u8; 2] = [0, 0];
        let mut trim = false;
        loop {
            let mut l = String::new();
            self.reader.read_line(&mut l).unwrap();
            let tmp = l.split_whitespace().find(|i| i.parse::<u8>().is_ok());
            if l.contains("EVENT") {
                if self.log || !trim {
                    writeln!(self.file, "{}", o).unwrap();
                }
                trim = true;
                o.clear();
                o += &(self.time.elapsed().unwrap().as_millis().to_string() + "|" + tmp.unwrap());

                k[0] = o.clone().split("|").collect::<Vec<&str>>()[1].parse::<u8>().unwrap();
            } else if l.contains("device:") { o += tmp.unwrap(); } else {
                if !&trim { o += &*l } else { o += l.trim() }
                if k[0] == 13u8 || k[0] == 14u8 || k[0] == 15u8 || k[0] == 16u8 {
                    if l.contains("detail:") {
                        k[1] = tmp.unwrap().parse::<u8>().unwrap();
                        self.tx.send(k).unwrap();
                    }
                }
            }
            if trim { o += "|" }
            l.clear();
        }
    }
}
