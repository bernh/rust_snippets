use std::fmt;

#[derive(Clone, Copy)]
struct Measurement {
    x: i32,
    y: i32,
}

const NUM_MEASUREMENTS: usize = 3;

pub struct Measurements {
    m: [Measurement; NUM_MEASUREMENTS],
}

impl Measurements {
    pub fn init() -> Self {
        let m = [Measurement { x: 0, y: 0 }; NUM_MEASUREMENTS];
        Self { m }
    }

    pub fn set(&mut self, offset: usize, x: i32, y: i32) {
        let p: Measurement = Measurement { x, y };
        if offset < self.m.len() {
            self.m[offset] = p;
        }
    }

    pub fn print(&self) -> () {
        println!("{}", self);
    }
}

impl fmt::Display for Measurements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, mes) in self.m.into_iter().enumerate() {
            writeln!(f, "Measurement {}: ({},{})", i + 1, mes.x, mes.y).unwrap();
        }
        write!(f, "")
    }
}
