use crate::utils::device::Device;
use vgtk::lib::gtk::{Orientation::Horizontal, *};
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct DevicesListItem {
    pub device: Device,
}

#[derive(Clone, Debug)]
pub enum Message {}

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

    fn update(&mut self, _event: Message) -> UpdateAction<Self> {
        UpdateAction::None
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <ListBoxRow>
                <Box orientation=Horizontal margin_top=10 margin_bottom=10>
                    <Image property_icon_name={match self.device.is_connected {
                        true => "emblem-ok-symbolic",
                        false => "window-close-symbolic"
                    }} Box::padding=10/>
                    <Label label=self.device.name.clone()></Label>
                </Box>
            </ListBoxRow>
        }
    }
}
