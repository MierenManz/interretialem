use std::fs::OpenOptions;
mod error;
mod module;
mod decoded_module;

fn main() {
    let mut open_options = OpenOptions::new();
    let mut _handle = open_options.read(true).open("mod.wasm").unwrap();
}
