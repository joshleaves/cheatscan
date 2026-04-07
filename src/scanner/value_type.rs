/// Primitive value kinds supported by the scanner.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueType {
  U8 = 0,
  U16 = 1,
  U32 = 2,
  I8 = 3,
  I16 = 4,
  I32 = 5,
  F32 = 6,
}

impl ValueType {
  /// Returns the byte width of this value type inside a RAM block.
  pub(crate) const fn width(self) -> usize {
    match self {
      Self::U8 | Self::I8 => 1,
      Self::U16 | Self::I16 => 2,
      Self::U32 | Self::I32 | Self::F32 => 4,
    }
  }
}
