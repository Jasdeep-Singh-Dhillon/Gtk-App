use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.run();
}
