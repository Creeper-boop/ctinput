use std::collections::HashMap;
use std::time::{SystemTime};

#[derive(Copy, Clone)]
struct Event {
    pub key_code: u8,
    pub action_code: u8,
    pub event_time: u128,
}

impl Event {
    fn new(key_code: u8, action_code: u8, time: SystemTime) -> Event {
        Event {
            key_code,
            action_code,
            event_time: time.elapsed().unwrap().as_millis(),
        }
    }
}

pub struct History {
    length: u8,
    width: u8,
    loc_x: u8,
    loc_y: u8,
    entries: Vec<Event>,
    key_dict: HashMap<u8, String>,
    system_time: SystemTime,
}

impl History {
    pub fn new(x: u8, y: u8, length: u8, width: u8) -> History {
        let time = SystemTime::now();

        History {
            length,
            width,
            loc_x: x,
            loc_y: y,
            entries: vec![Event::new(0, 0, time)].repeat(length as usize),
            key_dict: HashMap::new(),
            system_time: time,
        }
    }

    pub fn dict_add(&mut self, key_code: u8, key_name: String) -> u8 {
        if !self.key_dict.contains_key(&key_code) {
            self.key_dict.insert(key_code, key_name);
            return 0;
        }
        key_code
    }

    pub fn dict_remove(&mut self, key_code: u8) -> u8 {
        if self.key_dict.contains_key(&key_code) {
            self.key_dict.remove_entry(&key_code);
            return 0;
        }
        key_code
    }

    pub fn add_event(&mut self, key: u8, action: u8) {
        self.entries.push(Event::new(key, action, self.system_time));
        while self.entries.len() > self.length as usize {
            self.entries.remove(0);
        }
    }

    pub fn get_tui(&self, term_width: u16) -> String {
        let mut out = String::new();
        for i in 0..self.length {
            let mut row = String::new();
            if let Some(key) = self.key_dict.get(&self.entries[i as usize].key_code) {
                row += key
            } else { row += &self.entries[i as usize].key_code.to_string() }

            row += &*std::iter::repeat(" ").take(4 - row.len()).collect::<String>();

            match self.entries[i as usize].action_code {
                13u8 | 15u8 => row += "pus",
                14u8 | 16u8 => row += "rel",
                _ => row += &*self.entries[i as usize].action_code.to_string()
            };

            row += &*std::iter::repeat(" ").take(8 - row.len()).collect::<String>();

            row += &*self.entries[i as usize].event_time.to_string();

            row.truncate(self.width as usize);
            row.truncate((term_width - self.loc_x as u16 + 1) as usize);

            out += &("\x1b[".to_owned() + &*(self.loc_y + i).to_string() + ";" + &*self.loc_x.to_string() + "H" + &*row);
        }
        out
    }
}
