use sass_rs::*;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("src/views/App.css")
        .unwrap();
    file.set_len(0).unwrap();
    file.write(
        compile_file("src/views/App.scss", Options::default())
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
}
