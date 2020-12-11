use gtk::ListBoxRow;
use dbus::Message;
use dbus::blocking::Connection;
use crate::utils::conn_util::{with_conn, ConnVariant::*};
use crate::utils::device::{Device, DeviceBuilder};
use crate::views::devices_list::devices_list_item::DevicesListItem;
use std::collections::HashMap;
use relm_derive::{Msg, widget};
use relm::{ContainerWidget, Widget};
use crate::mconnect_dbus::{OrgMconnectDeviceManager, OrgFreedesktopDBusProperties, OrgMconnectDeviceManagerDeviceRemoved};
use relm::Update;
use std::path::PathBuf;
use gtk::prelude::*;
use self::Msg::*;

mod devices_list_item;

#[derive(Msg)]
pub enum Msg {
    AddDevice(PathBuf, Device),
    RemoveDevice(PathBuf),
    RowSelected(Option<ListBoxRow>)
}

pub struct Model {
    device_list_items: HashMap<PathBuf, ListBoxRow>,
}

#[widget]
impl Widget for DevicesList {
    fn model() -> Model {
        Model {
            device_list_items: HashMap::default(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            AddDevice(path, device) => {
                let widget = self.devices_list.add_widget::<DevicesListItem>(device);
                //HACK: need to store relm widget so that updates work. See https://github.com/antoyo/relm/issues/50#issuecomment-314931446
                self.model.device_list_items.insert(path, widget.widget().clone());
            },
            RemoveDevice(path) => self.devices_list.remove(
                                    self.model.device_list_items.get(&path).unwrap()),
            RowSelected(row) => {
                match row {
                    Some(row) => println!("{:?}", self.model.device_list_items.iter().find(|(_,v)| v.clone().clone() == row).unwrap().0),
                    None => println!("None")
                }
            }
        }
    }

    fn init_view(&mut self){
        with_conn(DeviceManager, |p| p.list_devices().unwrap())
            .iter()
            .map(|path| (PathBuf::from(path.to_string()), with_conn(Device(path), |p| p.get_all("org.mconnect.Device")).unwrap()))
            .map(|(path, map)| (path.clone(), DeviceBuilder::default().from_map(path, map).build().unwrap()))
            .for_each(|(path, device)|{
                self.update(AddDevice(path, device))
            });
        with_conn(DeviceManager, |p| p.match_signal(|h: OrgMconnectDeviceManagerDeviceRemoved, _: &Connection, _: &Message| {
            RemoveDevice(PathBuf::from(h.path));
            true
        })).unwrap();
    }

    view! {
        #[name="devices_list"]
        gtk::ListBox {
            row_selected(_,row) => Msg::RowSelected(row.map(|e| e.clone()))
        }
    }
}
