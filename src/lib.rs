mod ffi;
mod scanner;

pub use scanner::{ComparisonType, ScanError, Scanner, ValueType};

/// Byte order used when reading multi-byte values from a RAM block.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endianness {
  Little = 0,
  Big = 1,
}
impl TryFrom<u8> for Endianness {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Little),
      1 => Ok(Self::Big),
      _ => Err(()),
    }
  }
}

/// Address stepping strategy used when enumerating scan candidates.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alignment {
  Unaligned = 0,
  Aligned = 1
}
impl TryFrom<u8> for Alignment {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Unaligned),
      1 => Ok(Self::Aligned),
      _ => Err(()),
    }
  }
}



/// Typed comparison value used by scans.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScanValue {
  U8(u8),
  U16(u16),
  U32(u32),
  I8(i8),
  I16(i16),
  I32(i32),
  F32(f32),
  PreviousValue,
}


impl ScanValue {
  pub(crate) fn value_type(self) -> Option<ValueType> {
    match self {
      ScanValue::U8(_) => Some(ValueType::U8),
      ScanValue::U16(_) => Some(ValueType::U16),
      ScanValue::U32(_) => Some(ValueType::U32),
      ScanValue::I8(_) => Some(ValueType::I8),
      ScanValue::I16(_) => Some(ValueType::I16),
      ScanValue::I32(_) => Some(ValueType::I32),
      ScanValue::F32(_) => Some(ValueType::F32),
      ScanValue::PreviousValue => None,
    }
  }
}

/// Immutable scanner configuration shared across all scans.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Configuration {
  pub value_type: ValueType,
  pub endianness: Endianness,
  pub alignment: Alignment,
  pub base_address: u32,
}
