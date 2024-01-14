mod error;

use error::ValidationError;
use crate::decoded_ir::FuncType;
use crate::decoded_ir::Limits;
use crate::decoded_ir::ExportDescriptor;
use crate::decoded_ir::CodeBlock;
use crate::decoded_ir::indices::FuncIndex;
use crate::decoded_ir::indices::TypeIndex;

type Function = ();
type Memory = ();
type Table = ();
type Global = ();
type Export = ();

pub(crate) struct Module<'a> {
    functions: Box<[Function]>,
    // tables: Box<[Table]>,
    memory: Option<Memory>,
    globals: Box<[Global]>,
    // datas: Box<[u8]>,
    // datas: Vec<(u32, u32)>,
    start: Option<&'a Function>,
    exports: Box<[Export]>
}

pub(crate) struct ModuleBuilder {
    types: Vec<FuncType>,
    // imports: (),
    functions: Vec<TypeIndex>,
    // tables: (),
    memory: Option<Limits>,
    // globals: (),
    exports: Vec<ExportDescriptor>,
    start: Option<FuncIndex>,
    // elems: (),
    code: Vec<CodeBlock>
    // datas: (),
    // data_count: u32
}

impl ModuleBuilder {
    pub(crate) fn new() -> Self {
        Self {
            types: vec![],
            functions: vec![],
            memory: None,
            exports: vec![],
            start: None,
            code: vec![],
        }
    }

    pub(crate) fn add_type(&mut self, fn_type: FuncType) -> Result<(), ValidationError> {
        if self.types.len() == u32::MAX as usize {
            return Err(ValidationError::TooManyElements);
        }

        self.types.push(fn_type);
        Ok(())
    }

    pub(crate) fn add_func<T: Into<TypeIndex>>(&mut self, index: T) -> Result<(), ValidationError> {
        if self.functions.len() == u32::MAX as usize {
            return Err(ValidationError::TooManyElements);
        }

        self.functions.push(index.into());
        Ok(())
    }

    pub(crate) fn set_memory(&mut self, mem: Limits) -> Result<(), ValidationError> {
        if self.memory.is_some() {
            return Err(ValidationError::MultipleMemories);
        }

        self.memory = Some(mem);
        Ok(())
    }
}