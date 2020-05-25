extern crate gio;
extern crate gtk;
extern crate sourceview;
use gio::prelude::*;
use gtk::prelude::*;
use sourceview::prelude::*;

use std::env;

fn main() {
    let uiapp = gtk::Application::new(
        None,
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(320, 200);
        win.set_title("Basic example");

        let sourceview = sourceview::View::new();
        let completion = sourceview::CompletionWords::new(Some("Words"), None);
        completion.register(&sourceview.get_buffer().expect("Sourceview has no buffer."));
        sourceview.get_completion().expect("Sourceview has no completion").add_provider(&completion);
        win.add(&sourceview);

        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
