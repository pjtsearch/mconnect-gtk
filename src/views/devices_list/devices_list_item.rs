use gtk::Orientation::Horizontal;
use crate::utils::device::Device;
use gtk::Orientation::Vertical;
use relm::{Widget};
use relm_derive::{Msg, widget};
use gtk::prelude::*;

pub struct Model {
    device: Device
}

#[derive(Msg)]
pub enum Msg {
    
}

#[widget]
impl Widget for DevicesListItem {
    fn model(device: Device) -> Model {
        Model {
            device: device
        }
    }

    fn update(&mut self, event: Msg) {
        match event {

        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::ListBoxRow {
                gtk::Box {
                    orientation: Horizontal,
                    gtk::Label {
                        label: &self.model.device.name,
                        widget_name: "label",
                        text: &self.model.device.name,
                    },
                    gtk::Label {
                        label: &(" Connected: ".to_string() + &self.model.device.is_connected.to_string()),
                        widget_name: "label",
                        text: &(" Connected: ".to_string() + &self.model.device.is_connected.to_string()),
                    },
                }
            },
            gtk::Separator {}
        }
    }
}