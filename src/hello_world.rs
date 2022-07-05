use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connecting to "activate" singal of 'app'
    app.connect_activate(build_ui);

    // Running the application
    app.run();
}

fn build_ui(app: &Application) {
    // Adding button to the window
    let button = Button::builder()
        .label("Press me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connecting button to a click listener
    button.connect_clicked(move |button| {
        // Setting the label to "Hello World" when clicked
        button.set_label("Hello World");
    });

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Show the window
    window.present();
}