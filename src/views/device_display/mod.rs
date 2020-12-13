use crate::utils::device::Device;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct DeviceDisplay {
    pub device: Device
}

#[derive(Clone, Debug)]
pub enum Message {
    Allow,
    Disallow
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
            Message::Allow => {
                self.device.allow().unwrap();
                self.device = self.device.refreshed();
                UpdateAction::Render
            }
            Message::Disallow => {
                self.device.disallow().unwrap();
                self.device = self.device.refreshed();
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box orientation=Orientation::Vertical>
                
                <Label label=self.device.clone().name />
                {
                    gtk_if!(!self.device.allowed => {
                        <Button label="Allow" on clicked=|_| Message::Allow />
                    })
                }
                {
                    gtk_if!(self.device.allowed => {
                        <Button label="Disallow" on clicked=|_| Message::Disallow />
                    })
                }
            </Box>
        }
    }

}