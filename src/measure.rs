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
    for item in measurements.lock().unwrap().into_iter().enumerate() {
        let (i, p): (usize, Measurement) = item;
        println!("Measurement {}: ({},{})", i + 1, p.x, p.y);
    }
}

// mutable global state is an anti-pattern in Rust. You are basically forced to either
// - take care of synchronisation (Arc, Mutex, once_cell, ...)
// - or use "unsafe" to ignore all warnings

// Alternative version with same bug as C version (not ideomatic Rust!)
// for i in 0..4 {
//     let p = measurements[i];
//     println!("Measurement {}: ({},{})", i + 1, p.x, p.y);
// }
// Prints:
// Measurement 1: (0,0)
// Measurement 2: (2,3)
// Measurement 3: (5,5)
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 3', src/snippet_1.rs:29:17
