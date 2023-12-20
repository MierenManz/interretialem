#[derive(Clone, Copy, Debug)]
pub enum NumType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[derive(Clone, Copy, Debug)]
pub enum VecType {
    V128 = 0x7B,
}

#[derive(Clone, Copy, Debug)]
pub enum RefType {
    Func = 0x70,
    Extern = 0x6F,
}

#[derive(Clone, Copy, Debug)]
pub enum ValType {
    Num(NumType),
    Vec(VecType),
    Ref(RefType),
}
