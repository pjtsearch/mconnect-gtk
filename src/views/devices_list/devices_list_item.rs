use vgtk::Callback;
use crate::utils::device::Device;
use vgtk::lib::gtk::{prelude::*, Button, Orientation::Horizontal, Label, Box};
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct DevicesListItem {
    pub device: Device,
    pub on_clicked: Callback<Device>
}

#[derive(Clone, Debug)]
pub enum Message {
    OnClick
}

impl Component for DevicesListItem {
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
            Message::OnClick => {
                self.on_clicked.send(self.device.clone());
                UpdateAction::None
            }
            _ => UpdateAction::None
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Button on clicked=|_| Message::OnClick>
                <Box orientation=Horizontal>
                    <Label label=self.device.name.clone()></Label>
                    <Label label=" Connected: ".to_string() + &self.device.is_connected.to_string()></Label>
                </Box>
            </Button>
        }
    }

}