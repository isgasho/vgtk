extern crate gio;
extern crate glib;
extern crate gtk;

#[macro_use]
extern crate vgtk;

use gio::prelude::*;
use gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Button, ButtonBox, ButtonsType, DialogFlags, MessageDialog,
    MessageType, Orientation, Window,
};

fn not_much(b: &Button) {
    let window = b.get_toplevel().and_then(|w| w.downcast::<Window>().ok());
    let dialog = MessageDialog::new(
        window.as_ref(),
        DialogFlags::DESTROY_WITH_PARENT | DialogFlags::USE_HEADER_BAR,
        MessageType::Info,
        ButtonsType::Close,
        "Not much, dog, what's up with you?",
    );
    dialog.connect_response(|d, _| d.destroy());
    dialog.run();
}

fn activate(app: &gtk::Application) {
    let window: ApplicationWindow = gtk!{
        <ApplicationWindow(app) title="Updog",>
            <ButtonBox(Orientation::Horizontal)>
                <Button label="What's Updog?", on connect_clicked=not_much,/>
            </ButtonBox>
        </ApplicationWindow>
    };
    window.show_all();
}

fn main() {
    let app = Application::new("camp.lol.updog", ApplicationFlags::empty()).unwrap();
    app.connect_activate(activate);
    let args: Vec<String> = ::std::env::args().collect();
    ::std::process::exit(app.run(&args));
}
