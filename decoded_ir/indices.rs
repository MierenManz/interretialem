macro_rules! impl_index {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug)]
        pub(crate) struct $name(u32);
        impl $name {
            pub(crate) fn as_u32(&self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                Self(value)
            }
        }

        impl From<$name> for u32 {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

impl_index!(TypeIndex);
impl_index!(FuncIndex);
impl_index!(TableIndex);
impl_index!(MemoryIndex);
impl_index!(GlobalIndex);
impl_index!(ElemIndex);
impl_index!(DataIndex);
impl_index!(LocalIndex);
impl_index!(LabelIndex);
