#![recursion_limit = "512"]
#[macro_use]
extern crate derive_builder;

use crate::views::App;
use vgtk::run;

mod mconnect_dbus;
mod utils;
mod views;
mod components;

fn main() {
    std::process::exit(run::<App>());
}
