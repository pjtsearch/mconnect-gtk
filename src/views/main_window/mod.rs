// use crate::views::devices_list::DevicesList;
use crate::utils::device::Device;
use crate::utils::device_manager::DeviceManager;
use crate::utils::update_result::UpdateResult;
use crate::views::device_display::DeviceDisplay;
use crate::views::devices_list::DevicesList;
use crate::views::main_window::header_device_controls::HeaderDeviceControls;
use crate::views::main_window::notification::Notification;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, Component, UpdateAction, VNode};
mod header_device_controls;
mod notification;
mod share_file_btn;

#[derive(Clone, Default, Debug)]
pub struct MainWindow {
    devices: Option<Vec<Device>>,
    selected_device: Option<Device>,
    error: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Exit,
    DeviceSelected(std::boxed::Box<Device>),
    Refresh,
    UpdateError(Option<String>),
}

impl UpdateResult<Message> for MainWindow {
    fn update_result(&mut self, message: Message) -> Result<UpdateAction<Self>, dbus::Error> {
        match message {
            Message::Exit => {
                vgtk::quit();
                Ok(UpdateAction::None)
            }
            Message::DeviceSelected(device) => {
                self.selected_device = Some(device.as_ref().clone());
                Ok(UpdateAction::Render)
            }
            Message::Refresh => {
                self.devices = Some(DeviceManager.list_device_structs()?);
                if let Some(selected_device) = self.selected_device.clone() {
                    self.selected_device = Some(selected_device.refreshed()?);
                }
                Ok(UpdateAction::Render)
            }
            Message::UpdateError(error) => {
                self.error = error;
                Ok(UpdateAction::Render)
            }
        }
    }
}

impl Component for MainWindow {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, message: Message) -> UpdateAction<Self> {
        match self.update_result(message) {
            Ok(action) => {
                self.error = None;
                action
            }
            Err(err) => {
                self.error = Some(err.to_string());
                UpdateAction::Render
            }
        }
    }

    fn create(_props: Self::Properties) -> Self {
        match DeviceManager.list_device_structs() {
            Ok(devices) => MainWindow {
                devices: Some(devices),
                selected_device: None,
                error: None,
            },
            Err(err) => MainWindow {
                devices: None,
                selected_device: None,
                error: Some(err.to_string()),
            },
        }
    }

    fn view(&self) -> VNode<MainWindow> {
        gtk! {
             <Window on destroy=|_| Message::Exit>
                 <HeaderBar title="MConnect" show_close_button=true>
                     <Button image="view-refresh" on clicked=|_| Message::Refresh />
                     <Box HeaderBar::pack_type=PackType::End>
                         <@HeaderDeviceControls
                             selected_device=self.selected_device.clone()
                             on error=|e| Message::UpdateError(e)
                             on selected_device_change=|_| Message::Refresh />
                     </Box>
                 </HeaderBar>
                 <Box orientation=Orientation::Vertical>
                     <Box vexpand=true hexpand=true>
                         {gtk_if!(self.devices.is_some() => {
                             <@DevicesList
                                devices=self.devices.clone().unwrap()
                                on device_selected=|d| Message::DeviceSelected(std::boxed::Box::new(d))/>
                         })}
                         {gtk_if!(self.selected_device.is_some() => {
                             <ScrolledWindow hexpand=true>
                                 <@DeviceDisplay device=self.selected_device.clone().unwrap() />
                             </ScrolledWindow>
                         })}
                     </Box>
                     <@Notification text=self.error.clone() />
                 </Box>
             </Window>
        }
    }
}
