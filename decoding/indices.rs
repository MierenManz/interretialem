macro_rules! new_index {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug)]
        pub(crate) struct $name(u32);
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

new_index!(TypeIndex);
new_index!(FuncIndex);
new_index!(TableIndex);
new_index!(MemIndex);
new_index!(GlobalIndex);
new_index!(ElemIndex);
new_index!(DataIndex);
new_index!(LocalIndex);
new_index!(LabelIndex);