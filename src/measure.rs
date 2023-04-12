use std::sync::Mutex;

#[derive(Clone, Copy)]
struct Measurement {
    x: i32,
    y: i32,
}

const NUM_MEASUREMENTS: usize = 3;

static measurements: Mutex<[Measurement; NUM_MEASUREMENTS]> =
    Mutex::new([Measurement { x: 0, y: 0 }; NUM_MEASUREMENTS]);

pub fn set_measurement(offset: usize, x: i32, y: i32) -> () {
    let p: Measurement = Measurement { x, y };
    let mut data = measurements.lock().unwrap();
    data[offset] = p;
    // unlock called automatically when data goes out of scope}
}

pub fn print_measurements() -> () {
    for (i, mes) in measurements.lock().unwrap().into_iter().enumerate() {
        println!("Measurement {}: ({},{})", i + 1, mes.x, mes.y);
    }
}
