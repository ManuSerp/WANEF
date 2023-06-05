extern crate gtk;
use gtk::prelude::*;

use gtk::{Window, WindowType, Grid, Label,Button, Box};
use reqwest;

pub fn ui() -> Window {
    let main_window = Window::new(WindowType::Toplevel);
    main_window.set_title("WANEF");
    main_window.set_default_size(1000, 540);

    let vbox = Box::new(gtk::Orientation::Vertical, 0);
    let grid_toolbar = Grid::new();

    let button_settings = Button::with_label("Settings");
    let button_status = Button::with_label("Status");

    grid_toolbar.attach(&button_settings, 0, 0, 1, 1);
    grid_toolbar.attach(&button_status, 1, 0, 1, 1);
    vbox.pack_start(&grid_toolbar, false, false, 0);


    


    let grid = Grid::new();
    let label1 = Label::new(Some("Column 1"));
    let label2 = Label::new(Some("Column 2"));

    grid.attach(&label1, 0, 0, 1, 1); // add label1 to column 0
    grid.attach(&label2, 1, 0, 1, 1); // add label2 to column 1
    grid.set_column_homogeneous(true);
    vbox.pack_start(&grid, true, true, 0);

    main_window.add(&vbox);

    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    main_window

}

pub async fn fetch_api(url: String){
    let resp = reqwest::get(url).await.expect("request failed");
    assert!(resp.status().is_success());
    let json_data: serde_json::Value = resp
    .json()
    .await
    .expect("failed to parse JSON response");

println!("{:?}", json_data);


}