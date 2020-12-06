use crate::mconnect_dbus::OrgFreedesktopDBusProperties;
use crate::utils::conn_util::{with_conn, ConnVariant::*};
use gtk::{
    Inhibit,
    LabelExt,
    OrientableExt,
    WidgetExt,
};
use gtk::Orientation::Vertical;
use relm::{Component, ContainerWidget, Widget};
use relm_derive::{Msg, widget};
use mconnect_dbus::OrgMconnectDeviceManager;

use self::Msg::*;
mod mconnect_dbus;
mod utils;

pub struct DeviceListItemModel {
    device: String
}

#[derive(Msg)]
pub enum DeviceListItemMsg {
    
}

#[widget]
impl Widget for DeviceListItem {
    fn model(device: String) -> DeviceListItemModel {
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
                label: &self.model.device,
                widget_name: "label",
                text: &self.model.device,
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
        with_conn(DeviceManager, |p| p.list_devices().unwrap())
            .iter()
            .map(|path|
                    with_conn(Device(path), |p| p.get::<String>("org.mconnect.Device", "Name")))
            .for_each(|device|{
                let widget = self.devices_list.add_widget::<DeviceListItem>(device.unwrap());
                //HACK: need to store relm widget so that updates work. See https://github.com/antoyo/relm/issues/50#issuecomment-314931446
                self.model.device_list_items.push(widget.clone());
            });
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                #[name="devices_list"]
                gtk::Box {
                    orientation: Vertical,
                }
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}