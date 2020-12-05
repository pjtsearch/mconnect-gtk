use relm::{Widget};
use gtk::prelude::*;
use gtk::{Inhibit, Orientation::Vertical};
use relm_derive::Msg;
use relm_derive::widget;
use dbus::blocking::Connection;
mod mconnect_dbus;
use std::time::Duration;

#[derive(Msg)]
pub enum Msg {
    Decrement,
    Increment,
    Quit,
}

pub struct Model {
    counter: u32,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            // A call to self.label1.set_text() is automatically inserted by the
            // attribute every time the model.counter attribute is updated.
            Msg::Decrement => self.model.counter -= 1,
            Msg::Increment => self.model.counter += 1,
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Button {
                    // By default, an event with one paramater is assumed.
                    clicked => Msg::Increment,
                    // Hence, the previous line is equivalent to:
                    // clicked(_) => Increment,
                    label: "+",
                },
                gtk::Label {
                    // Bind the text property of this Label to the counter attribute
                    // of the model.
                    // Every time the counter attribute is updated, the text property
                    // will be updated too.
                    text: &self.model.counter.to_string(),
                },
                gtk::Button {
                    clicked => Msg::Decrement,
                    label: "-",
                },
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Win::run(()).unwrap();
    let c = Connection::new_session()?;
    let p = c.with_proxy("org.mconnect", "/org/mconnect/manager", Duration::new(5, 0));
    use mconnect_dbus::OrgMconnectDeviceManager;
    let devices:Result<Vec<dbus::Path<'static>>, dbus::Error> = p.list_devices();
    if let Ok(devices) = devices {
        devices.iter().for_each(|device| println!("{}", device.to_string()));
    }
    // println!("{:#?}", devices);
    Ok(())
}
