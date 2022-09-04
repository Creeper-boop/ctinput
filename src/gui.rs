/* ANSI for effect TODO OLD CODE FOR REFERENCE ONLY
    "\x1b["         being the escape char
    "2J"            clear
    "y;xH"          move the cursor to x, y
    "38;2;r;g;bm"   foreground color to r, g, b
    "48;2;r;g;bm"   background color to r, g, b
    "0m"            reset
    "nm"            setting the mode to n, being from 1..9
    thick "true"  thin "false"  lines
            ╔═══╗         ┌───┐
            ║   ║         │   │
    ╔═══╦═══╬═══╝ ┌───┬───┼───┘
    ║   ║   ║     │   │   │
    ╚═╦═╩═╦═╬═══╗ └─┬─┴─┬─┼───┐
      ║   ║ ║   ║   │   │ │   │
      ╠═══╣ ╚═══╝   ├───┤ └───┘
      ║   ║         │   │
      ╚═══╝         └───┘
 */

struct GraphicBox {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

pub struct Graphics {
    box_layer: Vec<GraphicBox>,
}

impl Graphics {
    pub fn new() -> Graphics {
        let box_layer: Vec<GraphicBox> = Vec::new();
        Graphics {
            box_layer,
        }
    }

    pub fn add(&mut self, x: u16, y: u16, w: u16, h: u16) {
        self.box_layer.push(GraphicBox { x, y, w, h })
    }

    pub fn generate(&mut self, double_lines: bool) -> String {
        let y = self.box_layer.iter().min_by_key(|e| e.y).unwrap().y;
        let x = self.box_layer.iter().min_by_key(|e| e.x).unwrap().x;
        let h = self.box_layer.iter().max_by_key(|e| e.y + e.h).unwrap();
        let w = self.box_layer.iter().max_by_key(|e| e.x + e.w).unwrap();

        let mut chars: Vec<Vec<u8>> = vec![vec![32u8; (w.x + w.w - x) as usize]; (h.h + h.y - y) as usize];

        for row in y..h.h + h.y {
            self.box_layer.iter().for_each(|e|
                if e.y == row || e.y + e.h - 1 == row {
                    vec![[32u8].repeat((e.x - x) as usize), [if double_lines { 144u8 } else { 128u8 }].repeat(e.w as usize)]
                } else if e.y < row && row < e.y + e.h - 1 {
                    vec![[32u8].repeat((e.x - x) as usize), vec![if double_lines { 145u8 } else { 130u8 }],
                         [32u8].repeat((e.w - 2) as usize), vec![if double_lines { 145u8 } else { 130u8 }]]
                } else { vec![] }.concat().iter().enumerate().for_each(|(i, e)| {
                    let tmp = chars.get((row - y) as usize).unwrap().get(i).unwrap();
                    if e != &32u8 && tmp != &145u8 && tmp != &130u8 {
                        *chars.get_mut((row - y) as usize).unwrap().get_mut(i).unwrap() = *e
                    }
                }
                )
            );
        }

        chars.clone().iter().enumerate().for_each(|(tmp_y, e)|
            e.iter().enumerate().for_each(|(tmp_x, &e)|
                if e != 32u8 { // double_lines makes multiple codes notably 172u8 represent 2 different symbols if problems arise fix that for if checks
                    if tmp_y > 0 && tmp_y < (h.y + h.h - y - 1) as usize {
                        if chars.get(tmp_y + 1).unwrap().get(tmp_x).unwrap() != &32u8 && chars.get(tmp_y - 1).unwrap().get(tmp_x).unwrap() != &32u8 {
                            if double_lines {
                                *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 145u8
                            } else {
                                *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 130u8
                            }
                        }
                    }
                    let e = *chars.get((tmp_y) as usize).unwrap().get(tmp_x).unwrap();
                    if tmp_y < (h.y + h.h - y - 1) as usize {
                        if chars.get(tmp_y + 1).unwrap().get(tmp_x).unwrap() != &32u8 {
                            if e == 144u8 || e == 128u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 166u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 172u8
                                }
                            }
                        }
                    }
                    let e = *chars.get((tmp_y) as usize).unwrap().get(tmp_x).unwrap();
                    if tmp_y > 0 {
                        if chars.get(tmp_y - 1).unwrap().get(tmp_x).unwrap() != &32u8 {
                            if e == 144u8 || e == 128u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 169u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 180u8
                                }
                            }
                            if e == 166u8 || (e == 172u8 && !double_lines) {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 172u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 188u8
                                }
                            }
                        }
                    }
                    let e = *chars.get((tmp_y) as usize).unwrap().get(tmp_x).unwrap();
                    if tmp_x < (w.x + w.w - x - 1) as usize {
                        if chars.get(tmp_y).unwrap().get(tmp_x + 1).unwrap() != &32u8 {
                            if e == 145u8 || e == 130u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 160u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 156u8
                                }
                            }
                        } else {
                            if e == 166u8 || (e == 172u8 && !double_lines) {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 151u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 144u8
                                }
                            }
                            if e == 169u8 || e == 180u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 157u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 152u8
                                }
                            }
                        }
                    }
                    let e = *chars.get((tmp_y) as usize).unwrap().get(tmp_x).unwrap();
                    if tmp_x > 0 {
                        if chars.get(tmp_y).unwrap().get(tmp_x - 1).unwrap() != &32u8 {
                            if e == 145u8 || e == 130u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 163u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 164u8
                                }
                            }
                            if e == 160u8 || e == 156u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 172u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 188u8
                                }
                            }
                        } else {
                            if e == 166u8 || (e == 172u8 && !double_lines) {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 148u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 140u8
                                }
                            }
                            if e == 169u8 || e == 180u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 154u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 148u8
                                }
                            }
                        }
                    }
                    let e = *chars.get((tmp_y) as usize).unwrap().get(tmp_x).unwrap();
                    if tmp_x == 0 {
                        if tmp_y > 0 && chars.get(tmp_y - 1).unwrap().get(tmp_x).unwrap() == &32u8 || tmp_y == 0 {
                            if e == 166u8 || e == 172u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 148u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 140u8
                                }
                            }
                        }
                        if tmp_y < (h.h + h.y - y - 1) as usize && chars.get(tmp_y + 1).unwrap().get(tmp_x).unwrap() == &32u8 || tmp_y == (h.h + h.y - y - 1) as usize {
                            if e == 169u8 || e == 180u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 154u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 148u8
                                }
                            }
                        }
                    }
                    if tmp_x == (w.x + w.w - x - 1) as usize {
                        if tmp_y > 0 && chars.get(tmp_y - 1).unwrap().get(tmp_x).unwrap() == &32u8 || tmp_y == 0 {
                            if e == 166u8 || e == 172u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 151u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 144u8
                                }
                            }
                        }
                        if tmp_y < (h.h + h.y - y - 1) as usize && chars.get(tmp_y + 1).unwrap().get(tmp_x).unwrap() == &32u8 || tmp_y == (h.h + h.y - y - 1) as usize {
                            if e == 169u8 || e == 180u8 {
                                if double_lines {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 157u8
                                } else {
                                    *chars.get_mut((tmp_y) as usize).unwrap().get_mut(tmp_x).unwrap() = 152u8
                                }
                            }
                        }
                    }
                }
            )
        );

        let mut out = String::new();
        chars.iter().enumerate().for_each(|(i, e)|
            {
                out += &*("\x1b[".to_string() + &(y + i as u16).to_string() + ";" + &x.to_string() + "H");
                e.iter().for_each(|e| out += &*String::from_utf8(
                    if *e == 32u8 { vec![*e] } else { vec![226u8, if double_lines { 149u8 } else { 148u8 }, *e] }).unwrap())
            }
        );
        out
    }
}
