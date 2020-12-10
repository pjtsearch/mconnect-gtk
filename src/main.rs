use std::env::args;
use gtk::prelude::*;
use gio::prelude::*;

pub fn main() {
    let application = gtk::Application::new(
        Some("com.pjtsearch.mconnect-gtk"),
        gio::ApplicationFlags::empty(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        let glade_src = include_str!("main_window.glade");
        let builder = gtk::Builder::from_string(glade_src);

        let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
        window.set_application(Some(app));

        window.show_all();
    });

    application.run(&args().collect::<Vec<_>>());
}