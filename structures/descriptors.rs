use super::instructions::WasmInstruction;
use super::types::*;

type Expr = Vec<WasmInstruction>;

pub struct Block {
    /// assert that the stack is this type
    pub block_type: BlockType,
    pub block_size: u32,
}

pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

pub enum ImportDescriptor {
    Func(u32),
    Table(TableType),
    Mem(MemoryType),
    Global(GlobalType),
}

pub struct Import {
    pub module: String,
    pub name: String,
    pub descriptor: ImportDescriptor,
}

pub struct Global {
    pub kind: GlobalType,
    pub init: Expr,
}

pub enum ExportDescriptor {
    Func,
    Table,
    Mem,
    Global,
}

pub struct Export {
    pub kind: ExportDescriptor,
    pub name: String,
    pub index: u32,
}

pub struct Function {
    locals: Vec<(u32, ValType)>,
    instructions: Expr,
}

pub struct CodeBody {
    size: u32,
    function: Function,
}

pub enum DataMode {
    /// `Active(memory_idx, offset)`
    Active(u32, u32),
    Passive,
}

pub struct Data {
    mode: DataMode,
    initial: Vec<u8>,
}
