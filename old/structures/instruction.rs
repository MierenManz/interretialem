use super::val_type::ValType;

pub const fn two_byte(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) | (b as u16)
}

#[derive(Clone, Copy, Debug)]
pub enum BlockType {
    Void,
    Value(ValType),
    TypeSignature(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    /// assert that the stack is this type
    pub block_type: BlockType,
    pub block_size: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

type TypeIndex = u32;
type FuncIndex = u32;
type TableIndex = u32;
type GlobalIndex = u32;
type ElemIndex = u32;
type DataIndex = u32;
type LocalIndex = u32;
type LabelIndex = u32;

#[derive(Clone, Debug)]
#[repr(u16)]
pub enum WasmInstruction {
    // Control flow instruction
    Unreachable = 0x00,
    Nop = 0x01,
    Block(Block) = 0x02,
    Loop(Block) = 0x03,
    If(Block) = 0x04,
    Else(Block) = 0x05,
    End = 0x0B,
    Br(LabelIndex) = 0x0C,
    BrIf(LabelIndex) = 0x0D,
    /// Last index of the slice is the default
    BrTable(Box<[LabelIndex]>) = 0x0E,
    Return = 0x0F,
    Call(FuncIndex) = 0x10,
    CallIndirect(TableIndex, TypeIndex) = 0x11,

    // Parametric Instructions
    Drop = 0x1A,
    Select = 0x1B,
    SelectT = 0x1C,

    // Variable Instructions
    LocalGet(LocalIndex) = 0x20,
    LocalSet(LocalIndex) = 0x21,
    LocalTee(LocalIndex) = 0x22,
    GlobalGet(GlobalIndex) = 0x23,
    GlobalSet(GlobalIndex) = 0x24,

    // Table Instructions
    TableGet(TableIndex) = 0x25,
    TableSet(TableIndex) = 0x26,
    TableInit(ElemIndex, TableIndex) = two_byte(0xFC, 12),
    ElemDrop(ElemIndex) = two_byte(0xFC, 13),
    TableCopy(TableIndex, TableIndex) = two_byte(0xFC, 14),
    TableGrow(TableIndex) = two_byte(0xFC, 15),
    TableSize(TableIndex) = two_byte(0xFC, 16),
    TableFill(TableIndex) = two_byte(0xFC, 17),

    // Memory Instructions
    I32Load(MemArg) = 0x28,
    I64Load(MemArg) = 0x29,
    F32Load(MemArg) = 0x2A,
    F64Load(MemArg) = 0x2B,
    I32Load8S(MemArg) = 0x2C,
    I32Load8U(MemArg) = 0x2D,
    I32Load16S(MemArg) = 0x2E,
    I32Load16U(MemArg) = 0x2F,
    I64Load8S(MemArg) = 0x30,
    I64Load8U(MemArg) = 0x31,
    I64Load16S(MemArg) = 0x32,
    I64Load16U(MemArg) = 0x33,
    I64Load32S(MemArg) = 0x34,
    I64Load32U(MemArg) = 0x35,
    I32Store(MemArg) = 0x36,
    I64Store(MemArg) = 0x37,
    F32Store(MemArg) = 0x38,
    F64Store(MemArg) = 0x39,
    I32Store8(MemArg) = 0x3A,
    I32Store16(MemArg) = 0x3B,
    I64Store8(MemArg) = 0x3C,
    I64Store16(MemArg) = 0x3D,
    I64Store32(MemArg) = 0x3E,
    MemorySize = two_byte(0x3F, 0x00),
    MemoryGrow = two_byte(0x40, 0x00),
    MemoryInit(DataIndex) = two_byte(0xFC, 8),
    DataDrop(DataIndex) = two_byte(0xFC, 9),
    MemoryCopy = two_byte(0xFC, 10),
    MemoryFill = two_byte(0xFC, 11),

    // Numeric Instructions
    I32Const(i32) = 0x41,
    I64Const(i64) = 0x42,
    F32Const(f32) = 0x43,
    F64Const(f64) = 0x44,

    I32Eqz = 0x45,
    I32Eq = 0x46,
    I32Ne = 0x47,
    I32LTS = 0x48,
    I32LTU = 0x49,
    I32GTS = 0x4A,
    I32GTU = 0x4B,
    I32LES = 0x4C,
    I32LEU = 0x4D,
    I32GES = 0x4E,
    I32GEU = 0x4F,

    I64Eqz = 0x50,
    I64Eq = 0x51,
    I64Ne = 0x52,
    I64LTS = 0x53,
    I64LTU = 0x54,
    I64GTS = 0x55,
    I64GTU = 0x56,
    I64LES = 0x57,
    I64LEU = 0x58,
    I64GES = 0x59,
    I64GEU = 0x5A,

    F32Eq = 0x5B,
    F32Ne = 0x5C,
    F32Lt = 0x5D,
    F32Gt = 0x5E,
    F32Le = 0x5F,
    F32Ge = 0x60,
    F64Eq = 0x61,
    F64Ne = 0x62,
    F64Lt = 0x63,
    F64Gt = 0x64,
    F64Le = 0x65,
    F64Ge = 0x66,

    I32Clz = 0x67,
    I32Ctz = 0x68,
    I32PopCnt = 0x69,
    I32Add = 0x6A,
    I32Sub = 0x6B,
    I32Mul = 0x6C,
    I32DivSigned = 0x6D,
    I32DivUnsigned = 0x6E,
    I32RemSigned = 0x6F,
    I32RemUnsigned = 0x70,
    I32And = 0x71,
    I32Or = 0x72,
    I32Xor = 0x73,
    I32ShiftLeft = 0x74,
    I32ShiftRSigned = 0x75,
    I32ShiftRUnsigned = 0x76,
    I32RotateLeft = 0x77,
    I32RotateRight = 0x78,

    I64Clz = 0x79,
    I64Ctz = 0x7A,
    I64PopCnt = 0x7B,
    I64Add = 0x7C,
    I64Sub = 0x7D,
    I64Mul = 0x7E,
    I64DivSigned = 0x7F,
    I64DivUnsigned = 0x80,
    I64RemSigned = 0x81,
    I64RemUnsigned = 0x82,
    I64And = 0x83,
    I64Or = 0x84,
    I64Xor = 0x85,
    I64ShiftLeft = 0x86,
    I64ShiftRSigned = 0x87,
    I64ShiftRUnsigned = 0x88,
    I64RotateLeft = 0x89,
    I64RotateRight = 0x8A,

    F32Abs = 0x8B,
    F32Neg = 0x8C,
    F32Ceil = 0x8D,
    F32Floor = 0x8E,
    F32Trunc = 0x8F,
    F32Nearest = 0x90,
    F32SQRT = 0x91,
    F32Add = 0x92,
    F32Sub = 0x93,
    F32Mul = 0x94,
    F32Div = 0x95,
    F32Min = 0x96,
    F32Max = 0x97,
    F32Copysign = 0x98,

    F64Abs = 0x99,
    F64Neg = 0x9A,
    F64Ceil = 0x9B,
    F64Floor = 0x9C,
    F64Trunc = 0x9D,
    F64Nearest = 0x9E,
    F64SQRT = 0x9F,
    F64Add = 0xA0,
    F64Sub = 0xA1,
    F64Mul = 0xA2,
    F64Div = 0xA3,
    F64Min = 0xA4,
    F64Max = 0xA5,
    F64Copysign = 0xA6,

    I32WrapI64 = 0xA7,
    I32TruncF32S = 0xA8,
    I32TruncF32U = 0xA9,
    I32TruncF64S = 0xAA,
    I32TruncF64U = 0xAB,
    I64ExtendI32S = 0xAC,
    I64ExtnedsI32U = 0xAD,
    I64TruncF32S = 0xAE,
    I64TruncF32U = 0xAF,
    I64TruncF64S = 0xB0,
    I64TruncF64U = 0xB1,
    F32ConvertI32S = 0xB2,
    F32ConvertI32U = 0xB3,
    F32ConvertI64S = 0xB4,
    F32ConvertI64U = 0xB5,
    F32DemoteF64 = 0xB6,
    F64ConvertI32S = 0xB7,
    F64ConvertI32U = 0xB8,
    F64ConvertI64S = 0xB9,
    F64ConvertI64U = 0xBA,
    F64PromoteF32 = 0xBB,
    I32ReinterpretF32 = 0xBC,
    I64ReinterpretF64 = 0xBD,
    F32ReinterpretI32 = 0xBE,
    F64ReinterpretI64 = 0xBF,
    I32Extends8S = 0xC0,
    I32Extends16S = 0xC1,
    I64Extends8S = 0xC2,
    I64Extends16S = 0xC3,
    I64Extends32S = 0xC4,
}
