use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button, Orientation};

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connecting to "activate" singal of 'app'
    app.connect_activate(build_ui);

    // Running the application
    app.run();
}

fn build_ui(app: &Application) {
    // Creating two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer for increment and decrement
    let number = Rc::new(Cell::new(0));

    // Connecting callbacks
    // When a button is clicked number value will change
    let number_copy = number.clone();
    button_increase.connect_clicked(move |_| number_copy.set(number_copy.get()+1));
    button_decrease.connect_clicked(move |_| number.set(number.get()-1)); 

    // Add Buttons to gtk_box
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Show the window
    window.present();
}