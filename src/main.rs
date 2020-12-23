#![recursion_limit = "512"]
#[macro_use]
extern crate derive_builder;

use crate::views::App;
use vgtk::run;

mod components;
mod mconnect_dbus;
mod utils;
mod views;

fn main() {
    std::process::exit(run::<App>());
}
