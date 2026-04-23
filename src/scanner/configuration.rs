use crate::scanner::{Alignment, Endianness, ValueType};

/// Immutable scanner configuration shared across all scans.
///
/// Address model:
/// - `base_address` is a 32-bit public address prefix used for reported results.
/// - the current public API is 32-bit for result addresses (`u32`).
/// - scanner construction fails with `ScanError::AddressOverflow` if candidate offsets combined
///   with `base_address` cannot be represented as `u32`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Configuration {
  pub value_type: ValueType,
  pub endianness: Endianness,
  pub alignment: Alignment,
  pub base_address: u32,
}
