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

impl TryFrom<u8> for ValueType {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::U8),
      1 => Ok(Self::U16),
      2 => Ok(Self::U32),
      3 => Ok(Self::I8),
      4 => Ok(Self::I16),
      5 => Ok(Self::I32),
      6 => Ok(Self::F32),
      _ => Err(()),
    }
  }
}