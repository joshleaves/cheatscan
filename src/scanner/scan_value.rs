use crate::scanner::value_type::ValueType;

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
