use super::error::RuntimeError;
use crate::structures::descriptors::Value;
use crate::structures::instruction::WasmInstruction;
use crate::structures::val_type::ValType;

use super::module::WasmModule;

struct FunctionCtx {
    pub locals: Box<[Value]>,
    pub stack: Vec<Value>,
    pub pc: u32,
    pub labels: Vec<u32>,
}

struct Function {
    type_id: u32,
    locals: Box<[ValType]>,
    code: Box<[WasmInstruction]>,
}

impl Function {
    pub fn new(type_id: u32, locals: Vec<ValType>, code: Box<[WasmInstruction]>) -> Self {
        Self {
            type_id,
            locals: locals.into(),
            code,
        }
    }

    pub fn call(
        &self,
        module: &mut WasmModule,
        ctx: FunctionCtx,
    ) -> Result<Vec<Value>, RuntimeError> {
        todo!();
    }
}
