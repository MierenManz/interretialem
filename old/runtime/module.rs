use super::descriptors::FnType;
// use crate::structures::descriptors::*;

type Function = ();
type Table = ();
type Memory = ();
type Global = ();
type Export = ();

pub struct  WasmModule {
    types: Box<[FnType]>,
    functions: Box<[Function]>,
}

// pub struct WasmModule {
//     types: Box<[TypeDescriptor]>,
//     functions: Box<[Function]>,
//     tables: Box<[Table]>,
//     memory: Option<Memory>,
//     globals: Box<[Global]>,
//     exports: Box<[Export]>,
//     start_fn: Option<u32>,
// }
