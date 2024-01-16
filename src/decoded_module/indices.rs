use binrw::binread;
use super::parsers::parse_varuint32;

macro_rules! impl_idx {
    ($name:ident) => {
        #[binread]
        #[derive(Clone, Copy)]
        pub(crate) struct $name {
            #[br(parse_with = parse_varuint32)]
            inner: u32,
        }
        impl $name {
            pub(crate) fn as_u32(&self) -> u32 {
                self.inner
            }
        }
    };
}

impl_idx!(TypeIndex);
impl_idx!(FuncIndex);
impl_idx!(TableIndex);
impl_idx!(MemIndex);
impl_idx!(GlobalIndex);
impl_idx!(ElemIndex);
impl_idx!(DataIndex);
impl_idx!(LocalIndex);
impl_idx!(LabelIndex);