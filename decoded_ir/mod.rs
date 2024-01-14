mod types;
mod error;
mod instructions;
pub(crate) mod indices;

pub(crate) use error::DecodingError;
pub(crate) use types::*;
pub(crate) use instructions::Instruction;

use self::indices::*;
#[repr(u8)]
pub (crate) enum ExportKind {
    Func(FuncIndex) = 0x00,
    Table(TableIndex) = 0x01,
    Mem(MemoryIndex) = 0x02,
    Global(GlobalIndex) = 0x03,
}
impl From<FuncIndex> for ExportKind {
    fn from(value: FuncIndex) -> Self {
        Self::Func(value)
    }
}
impl From<TableIndex> for ExportKind {
    fn from(value: TableIndex) -> Self {
        Self::Table(value)
    }
}
impl From<MemoryIndex> for ExportKind {
    fn from(value: MemoryIndex) -> Self {
        Self::Mem(value)
    }
}
impl From<GlobalIndex> for ExportKind {
    fn from(value: GlobalIndex) -> Self {
        Self::Global(value)
    }
}



pub(crate) struct ExportDescriptor {
    name: String,
    descriptor: ExportKind,
}
impl ExportDescriptor {
    fn new(name: String, descriptor: ExportKind) -> Self {
        Self {
            name,
            descriptor,
        }
    }
}

pub(crate) struct Local(ValType);
impl From<ValType> for Local {
    fn from(value: ValType) -> Self {
        Self(value)
    }
}

pub(crate) struct CodeBlock {
    locals: Vec<Local>,
    expr: Vec<Instruction>,
}

impl CodeBlock {
    pub(crate) fn new(expr: Vec<Instruction>) -> Self {
        Self::with_locals(expr, vec![])
    }

    pub(crate) fn with_locals(expr: Vec<Instruction>, locals: Vec<Local>) -> Self {
        Self {
            locals,
            expr,
        }
    }
}