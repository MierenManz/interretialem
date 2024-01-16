use std::fs::OpenOptions;
mod decoded_module;
mod error;
mod module;

fn main() {
    let mut open_options = OpenOptions::new();
    let mut _handle = open_options.read(true).open("mod.wasm").unwrap();
}
