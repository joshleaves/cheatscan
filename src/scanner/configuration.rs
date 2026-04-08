use crate::scanner::{Alignment, Endianness, ValueType};

/// Immutable scanner configuration shared across all scans.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Configuration {
  pub value_type: ValueType,
  pub endianness: Endianness,
  pub alignment: Alignment,
  pub base_address: u32,
}
