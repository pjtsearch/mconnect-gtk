use std::path::PathBuf;
use dbus::Message;
use dbus::blocking::Connection;
use crate::utils::conn_util::{with_conn, ConnVariant::*};
use crate::utils::device::{Device, DeviceBuilder};
use crate::views::devices_list::devices_list_item::DevicesListItem;
use std::collections::HashMap;
use vgtk::lib::gtk::{prelude::*, ListBox};
use vgtk::{gtk, Component, UpdateAction, VNode};
use crate::mconnect_dbus::{OrgMconnectDeviceManager, OrgFreedesktopDBusProperties, OrgMconnectDeviceManagerDeviceRemoved};

use self::DevicesListMessage::*;

mod devices_list_item;

#[derive(Clone, Debug)]
pub enum DevicesListMessage {
    AddDevice(PathBuf, Device),
    RemoveDevice(PathBuf)
}

#[derive(Clone, Debug, Default)]
pub struct DevicesList {
    devices: Vec<Device>
}

impl Component for DevicesList {
    type Message = DevicesListMessage;
    type Properties = ();

    fn update(&mut self, event: DevicesListMessage) -> UpdateAction<Self> {
        match event {
            AddDevice(path, device) => {
                self.devices.push(device); UpdateAction::Render
            }
            RemoveDevice(path) => {
                // self.devices.remove(
                //                     self.devices.get(&path).unwrap());
                UpdateAction::Render
            }
            // RowSelected(row) => {
            //     match row {
            //         Some(row) => println!("{:?}", self.model.device_list_items.iter().find(|(_,v)| v.clone().clone() == row).unwrap().0),
            //         None => println!("None")
            //     }
            // }
        }
    }

    fn create(_props: Self::Properties) -> Self{
        DevicesList {
            devices: with_conn(DeviceManager, |p| p.list_devices().unwrap())
                        .iter()
                        .map(|path| (PathBuf::from(path.to_string()), with_conn(Device(path), |p| p.get_all("org.mconnect.Device")).unwrap()))
                        .map(|(path, map)|DeviceBuilder::default().from_map(path, map).build().unwrap())
                        .collect()
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <ListBox>
                {
                    self.devices.iter().map(|dev| gtk! {
                        <@DevicesListItem device=dev />
                    })
                }
            </ListBox>
        }
    }
}
