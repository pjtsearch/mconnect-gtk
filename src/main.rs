#![recursion_limit="512"]
#[macro_use]
extern crate derive_builder;

use vgtk::run;
use crate::views::App;

mod views;
mod utils;
mod mconnect_dbus;

fn main() {
    std::process::exit(run::<App>());
}