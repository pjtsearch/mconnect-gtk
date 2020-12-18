// use crate::views::devices_list::DevicesList;
use crate::utils::device_manager::DeviceManager;
use crate::views::device_display::DeviceDisplay;
use std::path::PathBuf;
use crate::mconnect_dbus::OrgMconnectDeviceShare;
use crate::utils::device::Device;
use crate::views::devices_list::DevicesList;
use crate::views::main_window::share_file_btn::ShareFileBtn;
use vgtk::{ext::*, gtk, gtk_if, Component, UpdateAction, VNode};
use vgtk::lib::{gtk::*, gio::ApplicationFlags};
mod share_file_btn;

#[derive(Clone, Default, Debug)]
pub struct MainWindow {
    devices: Option<Vec<Device>>,
    selected_device: Option<Device>
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

impl Component for MainWindow {
   type Message = Message;
   type Properties = ();

   fn update(&mut self, message: Message) -> UpdateAction<Self> {
       match message {
            Message::Exit => {
               vgtk::quit();
               UpdateAction::None
            }
            Message::DeviceSelected(device) => {
               self.selected_device = Some(device);
               UpdateAction::Render
            }
            Message::AllowSelected => {
                self.selected_device.clone().map(|d|d.allow());
                self.selected_device = self.selected_device.clone().and_then(|d| d.refreshed().ok());
                UpdateAction::Render
            }
            Message::DisallowSelected => {
                self.selected_device.clone().map(|d|d.disallow());
                self.selected_device = self.selected_device.clone().and_then(|d| d.refreshed().ok());
                UpdateAction::Render
            }
            Message::Refresh => {
                self.devices = DeviceManager.list_device_structs().ok();
                self.selected_device = self.selected_device.clone().and_then(|d| d.refreshed().ok());
                UpdateAction::Render
            },
            Message::ShareFile(file) => {
                self.selected_device.clone().map(|d| d.share_file(file.to_str().unwrap()));
                UpdateAction::None
            }
       }
   }

   fn create(_props: Self::Properties) -> Self {
        MainWindow {
            devices: DeviceManager.list_device_structs().ok(),
            selected_device: None
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
                    <Box>
                        {gtk_if!(self.devices.is_some() => {
                            <@DevicesList devices=self.devices.clone().unwrap() on device_selected=|d| Message::DeviceSelected(d)/>
                        })}
                        {gtk_if!(self.selected_device.is_some() => {
                            <@DeviceDisplay device=self.selected_device.clone().unwrap() />
                        })}
                    </Box>
               </Window>
           </Application>
       }
   }
}