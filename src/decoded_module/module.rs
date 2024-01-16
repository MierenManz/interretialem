use binrw::binread;
use super::sections::*;


#[binread]
pub(crate) struct DecodedModule {
    type_section: TypeSection,
    import_section: ImportSection,
}
