#[macro_use]
extern crate derive_builder;

use relm::Widget;
use crate::views::MainWindow;

mod views;
mod utils;
mod mconnect_dbus;

fn main() {
    MainWindow::run(()).expect("Win::run failed");
}