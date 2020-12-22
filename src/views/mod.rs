use crate::views::main_window::MainWindow;
use vgtk::{VNode, Component, gtk, ext::*};
use vgtk::lib::{gtk::*, gio::ApplicationFlags};
use crate::utils::component_utils::*;

mod devices_list;
mod device_display;
mod main_window;

#[derive(Clone, Default, Debug)]
pub struct App {}

#[derive(Clone, Debug)]
pub enum Message {}
    
impl Component for App {
    type Message = Message;
    type Properties = ();
 
    fn view(&self) -> VNode<App> {
        gtk! {
            <Application::new_unwrap(Some("com.pjtsearch.mconnect-vgtk"), ApplicationFlags::empty()) css=include_str!("App.css")>
                <@MainWindow/>
            </Application>
        }
    }
 }