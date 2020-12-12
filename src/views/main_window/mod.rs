// use crate::views::devices_list::DevicesList;
use std::path::PathBuf;
use crate::utils::conn_util::{with_conn, ConnVariant::*};
use crate::mconnect_dbus::{OrgMconnectDeviceManager, OrgFreedesktopDBusProperties};
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
   DeviceSelected(Device)
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
               println!("{}", device.name);
               self.selected_device = Some(device);
               UpdateAction::Render
           }
       }
   }

   fn create(_props: Self::Properties) -> Self {
        MainWindow {
            devices: with_conn(DeviceManager, |p| p.list_devices().unwrap())
                .iter()
                .map(|path| (PathBuf::from(path.to_string()), with_conn(Device(path), |p| p.get_all("org.mconnect.Device")).unwrap()))
                .map(|(path, map)|DeviceBuilder::default().from_map(path, map).build().unwrap())
                .collect(),
            selected_device: None
        }
   }

   fn view(&self) -> VNode<MainWindow> {
       gtk! {
           <Application::new_unwrap(Some("com.pjtsearch.mconnect-vgtk"), ApplicationFlags::empty())>
               <Window on destroy=|_| Message::Exit>
                    <HeaderBar title="MConnect" show_close_button=true />
                    <Box>
                        <@DevicesList devices=self.devices.clone() on device_selected=|d| Message::DeviceSelected(d)/>
                        {
                            gtk_if!(self.selected_device.is_some() == true => {
                                <Label label=self.selected_device.clone().unwrap().name />
                            })
                        }
                    </Box>
               </Window>
           </Application>
       }
   }
}