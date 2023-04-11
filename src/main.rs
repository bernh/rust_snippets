#![allow(non_upper_case_globals)]
mod measure;

fn main() {
    measure::set_measurement(1, 2, 3);
    measure::set_measurement(2, 5, 5);
    measure::print_measurements();
}
