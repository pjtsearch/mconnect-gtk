use crate::utils::component_utils::*;
use crate::views::main_window::MainWindow;
use vgtk::lib::{gio::ApplicationFlags, gtk::*};
use vgtk::{ext::*, gtk, Component, VNode};

mod device_display;
mod devices_list;
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
