<<<<<<< HEAD
use std::sync::Mutex;

#[derive(Clone, Copy)]
=======
#[derive(Clone, Copy, Debug)]
>>>>>>> refs/heads/main
struct Measurement {
    x: i32,
    y: i32,
}

const NUM_MEASUREMENTS: usize = 3;

<<<<<<< HEAD
static measurements: Mutex<[Measurement; NUM_MEASUREMENTS]> =
    Mutex::new([Measurement { x: 0, y: 0 }; NUM_MEASUREMENTS]);

pub fn set_measurement(offset: usize, x: i32, y: i32) -> () {
    let p: Measurement = Measurement { x, y };
    let mut data = measurements.lock().unwrap();
    data[offset] = p;
    // unlock called automatically when data goes out of scope}
=======
#[derive(Debug)]
pub struct Measurements {
    m: [Measurement; NUM_MEASUREMENTS],
>>>>>>> refs/heads/main
}

<<<<<<< HEAD
pub fn print_measurements() -> () {
    for (i, mes) in measurements.lock().unwrap().into_iter().enumerate() {
        println!("Measurement {}: ({},{})", i + 1, mes.x, mes.y);
=======
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
>>>>>>> refs/heads/main
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
