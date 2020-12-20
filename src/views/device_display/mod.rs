use crate::utils::device::{Device, DeviceType};
use vgtk::lib::gtk::*;
use vgtk::{gtk, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct DeviceDisplay {
    pub device: Device
}

#[derive(Clone, Debug)]
pub enum Message {

}

impl DeviceDisplay {
    pub fn battery_icon<'a>(&self) -> String {
        let amount = match self.device.clone().battery_level {
            l if l < 1 => "empty",
            l if l < 15 => "caution",
            l if l < 25 => "low",
            l if l < 75 => "good",
            _ => "full"
        };
        let charging = match self.device.clone().battery_charging {
            true => "-charging",
            false => ""
        };
        format!("battery-{}{}", amount, charging)
    }
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

        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <ListBox margin_start=20 margin_end=20 margin_top=20 hexpand=true>
                <ListBoxRow>
                    <Box orientation=Orientation::Horizontal margin_start=5 margin_end=20>
                        <Image property_icon_name={match self.device.device_type {
                            DeviceType::Phone => "phone",
                            DeviceType::Desktop => "computer",
                            DeviceType::Tablet => "phone"
                        }} pixel_size=75 />
                        <Label label=self.device.clone().name />
                        <Image 
                            property_icon_name=self.battery_icon()
                            Box::pack_type=PackType::End 
                            pixel_size=50 />
                    </Box>
                </ListBoxRow>
            </ListBox>
        }
    }

}