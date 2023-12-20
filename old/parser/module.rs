use super::decoder::Decoder;
use super::error::*;
use std::io::Read;
use crate::structures::descriptors::*;

#[derive(Default)]
pub struct DecodedModule {
    fn_types: Vec<TypeDescriptor>,
    functions: Vec<u32>,
    tables: Vec<TableDescriptor>,
    memory: Option<MemoryLimits>,
    globals: Vec<GlobalDescriptor>,
    exports: Vec<ExportDescriptor>,
    start_fn: Option<u32>,
    // table_elements: Vec<Element>,
    // data_blobs: Vec<Data>,
    code_bodies: Vec<CodeBodyDescriptor>,
}

impl DecodedModule {
    fn check_limits(&mut self, limits: Limits) -> Result<(), ValidationError> {
        if limits.max.is_some_and(|x| x < limits.min) {
            return Err(ValidationError::MaxBiggerThanMin);
        }

        Ok(())
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_type(&mut self, descriptor: TypeDescriptor) {
        self.fn_types.push(descriptor)
    }

    pub fn add_import(&mut self) {
        todo!("Not implemented yet");
    }

    pub fn add_function(&mut self, type_idx: u32) -> Result<(), ValidationError> {
        if type_idx as usize >= self.fn_types.len() {
            return Err(ValidationError::UnknownIndex);
        }

        self.functions.push(type_idx);

        Ok(())
    }

    pub fn add_table(&mut self, table: TableDescriptor) -> Result<(), ValidationError> {
        self.check_limits(table.limits)?;
        self.tables.push(table);

        Ok(())
    }

    pub fn set_memory(&mut self, mem: MemoryLimits) -> Result<(), ValidationError> {
        let limits = mem.into();
        self.check_limits(limits)?;
        self.memory = Some(mem);
        Ok(())
    }

    pub fn add_global(&mut self, global: GlobalDescriptor) {
        self.globals.push(global);
    }

    pub fn add_export(&mut self, export: ExportDescriptor) -> Result<(), ValidationError> {
        match export.kind {
            ExportKind::Function(n) => {
                if n as usize >= self.functions.len() {
                    return Err(ValidationError::UnknownIndex);
                }
            }
            ExportKind::Global(n) => {
                if n as usize >= self.globals.len() {
                    return Err(ValidationError::UnknownIndex);
                }
            }
            ExportKind::Table(n) => {
                if n as usize >= self.tables.len() {
                    return Err(ValidationError::UnknownIndex);
                }
            }

            _ => {}
        };

        for x in &self.exports {
            if x.name == export.name {
                return Err(ValidationError::DuplicateExport);
            }
        }

        self.exports.push(export);

        Ok(())
    }

    pub fn set_start_fn(&mut self, start_idx: u32) -> Result<(), ValidationError> {
        if start_idx as usize >= self.functions.len() {
            return Err(ValidationError::UnknownIndex);
        }

        self.start_fn = Some(start_idx);

        Ok(())
    }

    pub fn add_element(&mut self) {
        todo!("Not yet implemented")
    }

    pub fn add_code_body(&mut self, body: CodeBodyDescriptor) -> Result<(), ValidationError> {
        if self.code_bodies.len() >= self.functions.len() {
            return Err(ValidationError::MoreBodiesThanFunctions);
        }
        self.code_bodies.push(body);

        Ok(())
    }

    pub fn add_data(&mut self, _data: DataDescriptor) -> Result<(), ValidationError> {
        todo!();
    }

    pub fn from_slice(mut slice: &[u8]) -> Result<Self, ParsingError> {
        Self::from_reader(&mut slice)
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Self, ParsingError> {
        let mut module = Self::new();
        let mut decoder = Decoder::new();
        decoder.decode_reader(reader, &mut module)?;
        Ok(module)
    }
}
