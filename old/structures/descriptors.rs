use super::instruction::WasmInstruction;
use super::val_type::RefType;
use super::val_type::ValType;

#[derive(Clone, Debug, Default)]
pub struct TypeDescriptor {
    pub params: Vec<ValType>,
    pub result: Vec<ValType>,
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryLimits {
    pub min: u16,
    pub max: Option<u16>,
}

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

impl From<MemoryLimits> for Limits {
    fn from(value: MemoryLimits) -> Self {
        Self {
            min: value.min.into(),
            max: value.max.and_then(|x| Some(x.into())),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TableDescriptor {
    pub limits: Limits,
    pub kind: RefType,
}

#[derive(Clone, Debug)]
pub struct GlobalDescriptor {
    pub kind: ValType,
    pub is_mut: bool,
    pub initial: Vec<WasmInstruction>,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ExportKind {
    Function(u32) = 0x00,
    Table(u32) = 0x01,
    Memory = 0x02,
    Global(u32) = 0x03,
}

pub struct ExportDescriptor {
    pub name: String,
    pub kind: ExportKind,
}

pub struct ElementDescriptor;

#[repr(u8)]
pub enum DataMode {
    Passive = 0x00,
    Active(u32) = 0x01,
}

pub struct DataDescriptor {
    pub mode: DataMode,
    pub init: Vec<u8>,
}

pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128([u8; 16]),
    FuncRef(u32),
    ExternRef(u32),
}

pub struct CodeBodyDescriptor {
    pub locals: Vec<ValType>,
    pub body: Vec<WasmInstruction>,
}
