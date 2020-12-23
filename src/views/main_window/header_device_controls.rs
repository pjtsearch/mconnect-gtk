use crate::mconnect_dbus::OrgMconnectDeviceShare;
use crate::utils::device::Device;
use crate::utils::update_result::UpdateResult;
use crate::views::main_window::share_file_btn::ShareFileBtn;
use std::path::PathBuf;
use vgtk::lib::gtk::*;
use vgtk::Callback;
use vgtk::{gtk, gtk_if, Component, UpdateAction, VNode};

#[derive(Clone, Debug, Default)]
pub struct HeaderDeviceControls {
    pub selected_device: Option<Device>,
    pub on_error: Callback<Option<String>>,
    pub on_selected_device_change: Callback<()>,
}

#[derive(Clone, Debug)]
pub enum Message {
    AllowSelected,
    DisallowSelected,
    ShareFile(PathBuf),
}

impl UpdateResult<Message> for HeaderDeviceControls {
    fn update_result(&mut self, event: Message) -> Result<UpdateAction<Self>, dbus::Error> {
        match event {
            Message::AllowSelected => {
                if let Some(selected_device) = self.selected_device.clone() {
                    selected_device.allow()?;
                    self.on_selected_device_change.send(());
                }
                Ok(UpdateAction::Render)
            }
            Message::DisallowSelected => {
                if let Some(selected_device) = self.selected_device.clone() {
                    selected_device.disallow()?;
                    self.on_selected_device_change.send(());
                }
                Ok(UpdateAction::Render)
            }
            Message::ShareFile(file) => {
                if let Some(selected_device) = self.selected_device.clone() {
                    selected_device.share_file(file.to_str().unwrap())?;
                }
                Ok(UpdateAction::Render)
            }
        }
    }
}

impl Component for HeaderDeviceControls {
    type Message = Message;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, message: Message) -> UpdateAction<Self> {
        match self.update_result(message) {
            Ok(action) => action,
            Err(err) => {
                self.on_error.send(Some(err.to_string()));
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box>
                {gtk_if!(self.selected_device.is_some() && !self.selected_device.clone().unwrap().allowed => {
                    <Button label="Allow" on clicked=|_| Message::AllowSelected />
                })}
                {gtk_if!(self.selected_device.is_some() && self.selected_device.clone().unwrap().allowed => {
                    <Button label="Disallow" on clicked=|_| Message::DisallowSelected />
                })}
                {gtk_if!(self.selected_device.is_some() && self.selected_device.clone().unwrap().is_connected => {
                    <Box>
                        <@ShareFileBtn on selected=|file| Message::ShareFile(file) />
                    </Box>
                })}
            </Box>
        }
    }
}
