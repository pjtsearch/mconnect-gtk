use crate::utils::conn_util::{ConnUtil, ConnVariant};
use gtk::{
    Inhibit,
    LabelExt,
    OrientableExt,
    WidgetExt,
};
use gtk::Orientation::{Horizontal, Vertical};
use relm::{Component, ContainerWidget, Widget};
use relm_derive::{Msg, widget};
use dbus::blocking::Connection;
use std::time::Duration;

use self::Msg::*;
mod mconnect_dbus;
mod utils;

pub struct DeviceListItemModel {
    device: dbus::Path<'static>
}

#[derive(Msg)]
pub enum DeviceListItemMsg {
    
}

#[widget]
impl Widget for DeviceListItem {
    fn model(device: dbus::Path<'static>) -> DeviceListItemModel {
        DeviceListItemModel {
            device: device
        }
    }

    fn update(&mut self, event: DeviceListItemMsg) {
        match event {

        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            gtk::Label {
                label: &self.model.device.to_string(),
                widget_name: "label",
                text: &self.model.device.to_string(),
            }
        }
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit
}

pub struct Model {
    device_list_items: Vec<Component<DeviceListItem>>,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {
            device_list_items: vec![],
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit()
        }
    }

    fn init_view(&mut self){
        ConnUtil::create_conn(ConnVariant::DeviceManager, move |p| {
            use mconnect_dbus::OrgMconnectDeviceManager;
            p.list_devices().unwrap().iter().for_each(|device|{
                let widget = self.hbox.add_widget::<DeviceListItem>(device.clone());
                //HACK: need to store relm widget so that updates work. See https://github.com/antoyo/relm/issues/50#issuecomment-314931446
                self.model.device_list_items.push(widget.clone());
            });
        });
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                #[name="hbox"]
                gtk::Box {
                    orientation: Horizontal,
                }
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}