extern crate gtk;
use gtk::prelude::*;
use wanef::ui;
fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let main_window = ui();

    main_window.show_all();
    gtk::main();
}


