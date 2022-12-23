use std::time::SystemTime;

pub struct Apm {
    width: u8,
    loc_x: u8,
    loc_y: u8,
    // average time per action in millis
    average: u128,
    // overall actions registered
    overall: u32,
    // last register in millis
    last_tick: u128,
    // time since apm was created
    system_time: SystemTime,
}

// todo this is bad please improve :)
impl Apm {
    pub fn new(x: u8, y: u8, width: u8) -> Apm {
        let time = SystemTime::now();

        Apm {
            width,
            loc_x: x,
            loc_y: y,
            average: 0,
            overall: 0,
            last_tick: time.elapsed().unwrap().as_millis(),
            system_time: time,
        }
    }

    pub fn tick(&mut self) {
        self.overall += 1;
        // update average time per action
        self.average = if self.average != 0 {
            (self.system_time.elapsed().unwrap().as_millis() - self.last_tick + self.average) / 2
        } else {
            self.system_time.elapsed().unwrap().as_millis() - self.last_tick
        };
        self.last_tick = self.system_time.elapsed().unwrap().as_millis();
    }

    pub fn get_tui(&self, term_width: u16) -> String {
        // lock the element width to available width or requested width
        let num_width: u8 = if self.width as u16 <= (term_width - self.loc_x as u16) {
            (self.width - 1) / 2
        } else {
            ((term_width - self.loc_x as u16 - 1) / 2) as u8
        };

        let mut average_apm = (60000f64 / self.average as f64).to_string();
        let mut overall_apm = (self.overall as f64 / (self.system_time.elapsed().unwrap().as_millis() as f64 / 60000f64)).to_string();

        average_apm.truncate(if (self.width - 1) % 2 == 0 { num_width } else { num_width + 1 } as usize);
        overall_apm.truncate(num_width as usize);

        "\x1b[".to_owned() + &self.loc_y.to_string() + ";" + &self.loc_x.to_string() + "H" + &*average_apm + "|" + &*overall_apm
    }
}