#[derive(Clone, Copy, Debug)]
struct Measurement {
    x: i32,
    y: i32,
}

const NUM_MEASUREMENTS: usize = 3;

#[derive(Debug)]
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
        println!("{:?}", self);
    }
}

// alternative version if we prefer a "functional" style
pub fn set_measurement(mes: &mut Measurements, offset: usize, x: i32, y: i32) -> () {
    let p: Measurement = Measurement { x, y };
    if offset < mes.m.len() {
        mes.m[offset] = p;
    }
}
