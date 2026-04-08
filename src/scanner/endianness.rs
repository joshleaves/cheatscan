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
