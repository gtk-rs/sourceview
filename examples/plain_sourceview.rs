extern crate gio;
extern crate gtk;
extern crate sourceview;
use gio::prelude::*;
use gtk::prelude::*;

use std::env;

fn main() {
    let uiapp = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(320, 200);
        win.set_title("Basic example");

        let sourceview = sourceview::View::new();
        win.add(&sourceview);

        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
