pub mod descriptors;
pub mod instructions;
pub mod types;

pub use descriptors::*;
pub use types::*;

type Section<T> = Vec<T>;

#[derive(Default, Debug)]
pub struct DecodedModule {
    pub(crate) magic_cookie: [u8; 4],
    pub(crate) version_bytes: [u8; 4],
    pub(crate) type_section: Section<FuncType>,
    pub(crate) import_section: Section<Import>,
    pub(crate) fn_section: Section<u32>,
    pub(crate) table_section: Section<TableType>,
    pub(crate) memory_section: Option<MemoryType>,
    pub(crate) global_section: Section<Global>,
    pub(crate) export_section: Section<Export>,
    pub(crate) start_section: Option<u32>,
    pub(crate) _element_section: (),
    pub(crate) code_section: Section<CodeBody>,
    pub(crate) data_section: Section<Data>,
    pub(crate) data_count: Option<u32>,
}
