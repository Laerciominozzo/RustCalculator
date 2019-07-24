mod calculator;

use std::process;
use calculator::Calculator;


fn main() {

    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let mut calculator = Calculator::new();
    let mut connected_calculator = calculator.connect_events();
    connected_calculator.borrow_mut().show_all();

    gtk::main();
}
