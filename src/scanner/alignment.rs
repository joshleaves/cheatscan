/// Address stepping strategy used when enumerating scan candidates.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alignment {
  Unaligned = 0,
  Aligned = 1,
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
