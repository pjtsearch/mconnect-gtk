#[macro_use]
extern crate derive_builder;

use vgtk::run;
use crate::views::MainWindow;

mod views;
mod utils;
mod mconnect_dbus;

fn main() {
    std::process::exit(run::<MainWindow>());
}