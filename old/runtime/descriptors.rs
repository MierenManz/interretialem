use crate::structures::val_type::ValType;

pub(crate) struct FnType {
  pub(crate) param_count: u32,
  pub(crate) signature: Vec<ValType>,
}