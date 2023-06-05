extern crate gtk;
use gtk::prelude::*;
use wanef::{ui, fetch_api};

#[tokio::main]
async fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let main_window = ui();
    fetch_api("https://api.open-meteo.com/v1/forecast?latitude=45.51&longitude=-73.59&hourly=temperature_2m".to_string()).await;
    main_window.show_all();
    gtk::main();

    
}


