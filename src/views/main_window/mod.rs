// use crate::views::devices_list::DevicesList;
use crate::utils::update_result::UpdateResult;
use crate::utils::device_manager::DeviceManager;
use crate::views::device_display::DeviceDisplay;
use std::path::PathBuf;
use crate::mconnect_dbus::OrgMconnectDeviceShare;
use crate::utils::device::Device;
use crate::views::devices_list::DevicesList;
use crate::views::main_window::share_file_btn::ShareFileBtn;
use crate::views::main_window::notification::Notification;
use vgtk::{ext::*, gtk, gtk_if, Component, UpdateAction, VNode};
use vgtk::lib::{gtk::*, gio::ApplicationFlags};
mod share_file_btn;
mod notification;

#[derive(Clone, Default, Debug)]
pub struct MainWindow {
    devices: Option<Vec<Device>>,
    selected_device: Option<Device>,
    error: Option<String>
}

#[derive(Clone, Debug)]
pub enum Message {
   Exit,
   DeviceSelected(Device),
   AllowSelected,
   DisallowSelected,
   ShareFile(PathBuf),
   Refresh
}

impl UpdateResult<Message> for MainWindow {
    fn update_result(&mut self, message: Message) -> Result<UpdateAction<Self>, dbus::Error> {
        match message {
            Message::Exit => {
               vgtk::quit();
               Ok(UpdateAction::None)
            }
            Message::DeviceSelected(device) => {
               self.selected_device = Some(device);
               Ok(UpdateAction::Render)
            }
            Message::AllowSelected => {
                if let Some(selected_device) = self.selected_device.clone() {
                    selected_device.allow()?;
                    self.selected_device = Some(selected_device.refreshed()?);
                }
                Ok(UpdateAction::Render)
            }
            Message::DisallowSelected => {
                if let Some(selected_device) = self.selected_device.clone() {
                    selected_device.disallow()?;
                    self.selected_device = Some(selected_device.refreshed()?);
                }
                Ok(UpdateAction::Render)
            }
            Message::Refresh => {
                self.devices = Some(DeviceManager.list_device_structs()?);
                if let Some(selected_device) = self.selected_device.clone() {
                    self.selected_device = Some(selected_device.refreshed()?);
                }
                Ok(UpdateAction::Render)
            },
            Message::ShareFile(file) => {
                if let Some(selected_device) = self.selected_device.clone() {
                    selected_device.share_file(file.to_str().unwrap())?;
                }
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
               error: None
           },
           Err(err) => MainWindow {
                devices: None,
                selected_device: None,
                error: Some(err.to_string())
            }
       }
   }

   fn view(&self) -> VNode<MainWindow> {
       gtk! {
           <Application::new_unwrap(Some("com.pjtsearch.mconnect-vgtk"), ApplicationFlags::empty())>
               <Window on destroy=|_| Message::Exit>
                    <HeaderBar title="MConnect" show_close_button=true>
                        <Button image="view-refresh" on clicked=|_| Message::Refresh />
                        {gtk_if!(self.selected_device.is_some() && self.selected_device.clone().unwrap().allowed => {
                            <Button HeaderBar::pack_type=PackType::End label="Disallow" on clicked=|_| Message::DisallowSelected />
                        })}
                        {gtk_if!(self.selected_device.is_some() && self.selected_device.clone().unwrap().is_connected => {
                            <Box HeaderBar::pack_type=PackType::End>
                                <@ShareFileBtn on selected=|file| Message::ShareFile(file) />
                            </Box>
                        })}
                        {gtk_if!(self.selected_device.is_some() && !self.selected_device.clone().unwrap().allowed => {
                            <Button label="Allow" HeaderBar::pack_type=PackType::End on clicked=|_| Message::AllowSelected />
                        })} 
                    </HeaderBar>
                    <Box orientation=Orientation::Vertical>
                        <Box vexpand=true>
                            {gtk_if!(self.devices.is_some() => {
                                <@DevicesList devices=self.devices.clone().unwrap() on device_selected=|d| Message::DeviceSelected(d)/>
                            })}
                            {gtk_if!(self.selected_device.is_some() => {
                                <@DeviceDisplay device=self.selected_device.clone().unwrap() />
                            })}
                        </Box>
                        <@Notification text=self.error.clone() />
                    </Box>
               </Window>
           </Application>
       }
   }
}