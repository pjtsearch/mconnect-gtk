// use crate::views::devices_list::DevicesList;
use crate::views::device_display::DeviceDisplay;
use std::path::PathBuf;
use crate::utils::conn_util::{with_conn, ConnVariant::*};
use crate::mconnect_dbus::{OrgMconnectDeviceManager, OrgMconnectDeviceShare};
use crate::utils::device::{Device, DeviceBuilder};
use crate::views::devices_list::DevicesList;
use vgtk::{ext::*, gtk, gtk_if, Component, UpdateAction, VNode};
use vgtk::lib::{gtk::*, gio::ApplicationFlags};

#[derive(Clone, Default, Debug)]
pub struct MainWindow {
    devices: Vec<Device>,
    selected_device: Option<Device>
}

#[derive(Clone, Debug)]
pub enum Message {
   Exit,
   DeviceSelected(Device),
   AllowSelected,
   DisallowSelected,
   ShareFile,
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
                self.selected_device.clone().unwrap().allow().unwrap();
                self.selected_device = self.selected_device.clone().map(|d| d.refreshed());
                UpdateAction::Render
            }
            Message::DisallowSelected => {
                self.selected_device.clone().unwrap().disallow().unwrap();
                self.selected_device = self.selected_device.clone().map(|d| d.refreshed());
                UpdateAction::Render
            }
            Message::Refresh => {
                self.devices = with_conn(DeviceManager, |p| p.list_devices().unwrap())
                                .iter()
                                .map(|path| 
                                        with_conn(
                                            Device(path), 
                                            |p| DeviceBuilder::default().from_proxy(PathBuf::from(path.to_string()), p).build().unwrap()))
                                .collect();
                self.selected_device = self.selected_device.clone().map(|d| d.refreshed());
                UpdateAction::Render
            },
            Message::ShareFile => {
                let dialog = FileChooserDialog::with_buttons::<Window>(
                    Some("Open File"),
                    None,
                    FileChooserAction::Open,
                    &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)]
                );
                let result = dialog.run();
                if result == ResponseType::Accept {
                    if let Some(uri) = dialog.get_filename() {
                        println!("{:?}", self.selected_device.clone().and_then(|d| d.share_file(uri.to_str().unwrap()).ok()).unwrap());
                    }
                }
                dialog.close();
                UpdateAction::None
            }
       }
   }

   fn create(_props: Self::Properties) -> Self {
        MainWindow {
            devices: with_conn(DeviceManager, |p| p.list_devices().unwrap())
                .iter()
                .map(|path| 
                        with_conn(
                            Device(path), 
                            |p| DeviceBuilder::default().from_proxy(PathBuf::from(path.to_string()), p).build().unwrap()))
                .collect(),
            selected_device: None
        }
   }

   fn view(&self) -> VNode<MainWindow> {
       gtk! {
           <Application::new_unwrap(Some("com.pjtsearch.mconnect-vgtk"), ApplicationFlags::empty())>
               <Window on destroy=|_| Message::Exit>
                    <HeaderBar title="MConnect" show_close_button=true>
                        <Button image="view-refresh" on clicked=|_| Message::Refresh />
                        {
                            gtk_if!(self.selected_device.is_some() && self.selected_device.clone().unwrap().allowed => {
                                <Box HeaderBar::pack_type=PackType::End>
                                    <Button label="Share File" on clicked=|_| Message::ShareFile />
                                    <Button label="Disallow" on clicked=|_| Message::DisallowSelected />
                                </Box>
                            })
                        }
                        {
                            gtk_if!(self.selected_device.is_some() && !self.selected_device.clone().unwrap().allowed => {
                                <Button label="Allow" HeaderBar::pack_type=PackType::End on clicked=|_| Message::AllowSelected />
                            })
                        } 
                    </HeaderBar>
                    <Box>
                        <@DevicesList devices=self.devices.clone() on device_selected=|d| Message::DeviceSelected(d)/>
                        {
                            gtk_if!(self.selected_device.is_some() == true => {
                                <@DeviceDisplay device=self.selected_device.clone().unwrap() />
                            })
                        }
                    </Box>
               </Window>
           </Application>
       }
   }
}