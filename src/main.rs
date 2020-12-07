use relm::Update;
use crate::utils::device::Device;
use mconnect_dbus::OrgFreedesktopDBusProperties;
use utils::conn_util::{with_conn, ConnVariant::*};
#[macro_use]
extern crate derive_builder;
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
use utils::device::DeviceBuilder;

use self::Msg::*;
mod mconnect_dbus;
mod utils;

pub struct DeviceListItemModel {
    device: Device
}

#[derive(Msg)]
pub enum DeviceListItemMsg {
    
}

#[widget]
impl Widget for DeviceListItem {
    fn model(device: Device) -> DeviceListItemModel {
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
                label: &self.model.device.name,
                widget_name: "label",
                text: &self.model.device.name,
            }
        }
    }
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    AddDevice(String, Device),
    RemoveDevice(String),
}

pub struct Model {
    device_list_items: Vec<(String, Component<DeviceListItem>)>,
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
            Quit => gtk::main_quit(),
            AddDevice(path, device) => {
                let widget = self.devices_list.add_widget::<DeviceListItem>(device);
                //HACK: need to store relm widget so that updates work. See https://github.com/antoyo/relm/issues/50#issuecomment-314931446
                self.model.device_list_items.push((path, widget.clone()));
            },
            RemoveDevice(path) => {
               self.devices_list.remove_widget(
                    self.model.device_list_items.iter().find(move |(d_path,_)| d_path.to_owned() == path).unwrap().1.clone());
            }
        }
    }

    fn init_view(&mut self){
        with_conn(DeviceManager, |p| p.list_devices().unwrap())
            .iter()
            .map(|path| (path.to_string(), with_conn(Device(path), |p| p.get_all("org.mconnect.Device")).unwrap()))
            .map(|(path, map)| (path, DeviceBuilder::default().from_map(map).build().unwrap()))
            .for_each(|(path, device)|{
                self.update(AddDevice(path, device))
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