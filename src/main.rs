#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]

use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

// We're using gtk-builder feature here
use gtk_ui_builder::prelude::*;

fn main() {
    gtk4::init().expect("GTK initialization failed");
    libadwaita::init().expect("adw initialization failed");

    // Create app
    let application = gtk::Application::new(
        Some("com.github.krypt0nn.gtk-ui-builder"),
        Default::default()
    );

    // Init app window and show it
    application.connect_activate(|app| {
        // You also can parse blueprint with Parser::parse
        // and then use it in gtk4::Builder
        let builder = Builder::new(include_str!("../assets/ui/main.blp"))
            .expect("Failed to parse blueprint");

        let window = builder.object::<adw::ApplicationWindow>("window").unwrap();

        window.set_application(Some(app));
        window.show();
    });

    // Run app
    application.run();
}