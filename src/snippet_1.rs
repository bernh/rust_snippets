#[derive(Clone, Copy)]
struct Measurement {
    x: i32,
    y: i32,
}

const NUM_MEASUREMENTS: usize = 3;

static mut measurements: [Measurement; NUM_MEASUREMENTS] =
    [Measurement { x: 0, y: 0 }; NUM_MEASUREMENTS]; // initialization enforced!

pub fn set_point(offset: usize, x: i32, y: i32) -> () {
    let p: Measurement = Measurement { x, y };
    unsafe {
        // mutable global state is an anti-pattern in Rust. You are basically forced to either
        // - take care of synchronisation (Arc, Mutex, once_cell, ...)
        // - or use "unsafe" to ignore all warnings
        measurements[offset] = p;
    }
}

pub fn print_points() -> () {
    unsafe {
        for item in measurements.into_iter().enumerate() {
            let (i, p): (usize, Measurement) = item;
            println!("Measurement {}: ({},{})", i + 1, p.x, p.y);
        }
    }
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
}
