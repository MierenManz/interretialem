use std::fs::OpenOptions;
mod module;
mod decoded_ir;
fn main() {
    let mut open_options = OpenOptions::new();
    let mut _handle = open_options.read(true).open("mod.wasm").unwrap();
}
