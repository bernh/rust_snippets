#![allow(non_upper_case_globals)]
mod measure;
mod measure_impl;

use measure::Measurements;

fn main() {
<<<<<<< HEAD
    // module level global variable and functions
    measure::set_measurement(1, 2, 3);
    measure::set_measurement(2, 5, 5);
    measure::print_measurements();

    // alternative version using a struct and impl for set and print
    let mut mes = measure_impl::Measurements::init();
    mes.set(1, 2, 3);
    mes.set(2, 5, 5);
=======
    let mut mes = Measurements::init();

    mes.set(1, 2, 3);
    mes.set(2, 5, 5);
    mes.print();

    measure::set_measurement(&mut mes, 0, 7, 7);
>>>>>>> refs/heads/main
    mes.print();
}
