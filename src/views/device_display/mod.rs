use crate::utils::device::Device;
use vgtk::lib::gtk::{*, Orientation::Horizontal};
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct DeviceDisplay {
    pub device: Device
}

#[derive(Clone, Debug)]
pub enum Message {

}

impl Component for DeviceDisplay {
    type Message = Message;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, event: Message) -> UpdateAction<Self> {
        match event {
            _ => UpdateAction::None
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Label label=self.device.clone().name />
        }
    }

}