// use crate::views::devices_list::DevicesList;
use crate::views::devices_list::DevicesList;
use vgtk::{ext::*, gtk, Component, UpdateAction, VNode};
use vgtk::lib::{gtk::*, gio::ApplicationFlags};

#[derive(Clone, Default, Debug)]
pub struct MainWindow {
    counter: usize,
}

#[derive(Clone, Debug)]
pub enum Message {
   Exit
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
       }
   }

   fn view(&self) -> VNode<MainWindow> {
       gtk! {
           <Application::new_unwrap(Some("com.pjtsearch.mconnect-vgtk"), ApplicationFlags::empty())>
               <Window on destroy=|_| Message::Exit>
                   <HeaderBar title="MConnect" show_close_button=true />
                    <@DevicesList/>
               </Window>
           </Application>
       }
   }
}