mod measure;

use measure::Measurements;

fn main() {
    let mut mes = Measurements::init();

    mes.set(1, 2, 3);
    mes.set(2, 5, 5);
    mes.print();

    measure::set_measurement(&mut mes, 0, 7, 7);
    mes.print();
}
