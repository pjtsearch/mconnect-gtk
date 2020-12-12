use vgtk::Callback;
use crate::utils::device::Device;
use crate::views::devices_list::devices_list_item::DevicesListItem;
use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, UpdateAction, VNode};
use std::convert::TryFrom;

use self::DevicesListMessage::*;

mod devices_list_item;

#[derive(Clone, Debug)]
pub enum DevicesListMessage {
    RowSelected(Option<i32>)
}

#[derive(Clone, Debug, Default)]
pub struct DevicesList {
    pub devices: Vec<Device>,
    pub on_device_selected: Callback<Device>
}

impl Component for DevicesList {
    type Message = DevicesListMessage;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, event: DevicesListMessage) -> UpdateAction<Self> {
        match event {
            RowSelected(row) => {
                if let Some(row) = row {
                    let device = self.devices[usize::try_from(row).unwrap()].clone();
                    self.on_device_selected.send(device);
                }
                UpdateAction::None
            }
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
