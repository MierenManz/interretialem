use std::num::NonZeroU32;

use super::instructions::WasmInstruction;
use super::types::*;

type Expr = Vec<WasmInstruction>;

#[derive(Debug)]
pub struct Block {
    /// assert that the stack is this type
    pub block_type: BlockType,
    pub block_size: NonZeroU32,
}

#[derive(Debug)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

#[derive(Debug)]
pub enum ImportDescriptor {
    Func(u32),
    Table(TableType),
    Mem(MemoryType),
    Global(GlobalType),
}

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub descriptor: ImportDescriptor,
}

#[derive(Debug)]
pub struct Global {
    pub kind: GlobalType,
    pub init: Expr,
}

#[derive(Debug)]
pub enum ExportDescriptor {
    Func,
    Table,
    Mem,
    Global,
}

#[derive(Debug)]
pub struct Export {
    pub kind: ExportDescriptor,
    pub name: String,
    pub index: u32,
}

#[derive(Debug)]
pub struct Function {
    pub locals: Vec<(NonZeroU32, ValType)>,
    pub instructions: Expr,
}

#[derive(Debug)]
pub struct CodeBody {
    pub size: u32,
    pub function: Function,
}

#[derive(Debug)]
pub enum DataMode {
    ActiveMemZero(u32),
    Active(NonZeroU32, u32),
    Passive,
}

#[derive(Debug)]
pub struct Data {
    mode: DataMode,
    initial: Vec<u8>,
}
