use crate::views::devices_list::DevicesList;
use crate::views::main_window::header::Header;
use relm::init;
use gtk::{
    Inhibit,
    OrientableExt,
    WidgetExt,
};
use gtk::Orientation::Vertical;
use relm::{Widget};
use relm_derive::{Msg, widget};
use gtk::prelude::*;
use self::Msg::*;
mod header;

#[derive(Msg)]
pub enum Msg {
    Quit
}

pub struct Model {}

#[widget]
impl Widget for MainWindow {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit()
        }
    }

    view! {
        gtk::Window {
            titlebar: Some(init::<Header>("MConnect".to_string()).unwrap().widget()),
            gtk::Box {
                orientation: Vertical,
                DevicesList {}
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}