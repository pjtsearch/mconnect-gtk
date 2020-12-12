use std::path::PathBuf;
use crate::utils::conn_util::{with_conn, ConnVariant::*};
use crate::utils::device::{Device, DeviceBuilder};
use crate::views::devices_list::devices_list_item::DevicesListItem;
use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, UpdateAction, VNode};
use crate::mconnect_dbus::{OrgMconnectDeviceManager, OrgFreedesktopDBusProperties};
use std::convert::TryFrom;

use self::DevicesListMessage::*;

mod devices_list_item;

#[derive(Clone, Debug)]
pub enum DevicesListMessage {
    RowSelected(Option<i32>)
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
            RowSelected(row) => {
                match row {
                    Some(row) => {
                        let device = self.devices[usize::try_from(row).unwrap()].clone();
                        println!("{:#?}", device);
                        UpdateAction::None
                    },
                    None => UpdateAction::None
                }
            }
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
            <ListBox on row_selected=|_,row| DevicesListMessage::RowSelected(row.map(|r|r.to_owned().get_index()))>
                {
                    self.devices.iter().map(|dev| gtk! {
                        <@DevicesListItem device=dev/>
                    })
                }
            </ListBox>
        }
    }
}
