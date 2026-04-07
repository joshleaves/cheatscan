/// Comparison operator applied during a scan.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonType {
  Eq = 0,
  Ne = 1,
  Lt = 2,
  Le = 3,
  Gt = 4,
  Ge = 5,
}

impl ComparisonType {
  /// Returns comparison function for two values.
  pub(crate) fn to_fn<T>(self) -> fn(T, T) -> bool
  where
    T: PartialOrd + PartialEq,
  {
    match self {
      Self::Eq => |lhs, rhs| lhs == rhs,
      Self::Ne => |lhs, rhs| lhs != rhs,
      Self::Lt => |lhs, rhs| lhs < rhs,
      Self::Le => |lhs, rhs| lhs <= rhs,
      Self::Gt => |lhs, rhs| lhs > rhs,
      Self::Ge => |lhs, rhs| lhs >= rhs,
    }
  }
}

impl TryFrom<u8> for ComparisonType {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Eq),
      1 => Ok(Self::Ne),
      2 => Ok(Self::Lt),
      3 => Ok(Self::Le),
      4 => Ok(Self::Gt),
      5 => Ok(Self::Ge),
      _ => Err(()),
    }
  }
}