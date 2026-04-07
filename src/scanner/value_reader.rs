use crate::Endianness;

/// Reads a `u8` from a raw byte slice.
pub fn read_u8(bytes: &[u8], _endianness: Endianness) -> u8 {
  bytes[0]
}

/// Reads a `u16` from a raw byte slice using the configured endianness.
pub fn read_u16(bytes: &[u8], endianness: Endianness) -> u16 {
  let bytes = [bytes[0], bytes[1]];
  match endianness {
    Endianness::Little => u16::from_le_bytes(bytes),
    Endianness::Big => u16::from_be_bytes(bytes),
  }
}

/// Reads a `u32` from a raw byte slice using the configured endianness.
pub fn read_u32(bytes: &[u8], endianness: Endianness) -> u32 {
  let bytes = [
    bytes[0],
    bytes[1],
    bytes[2],
    bytes[3],
  ];
  match endianness {
    Endianness::Little => u32::from_le_bytes(bytes),
    Endianness::Big => u32::from_be_bytes(bytes),
  }
}

/// Reads an `i8` from a raw byte slice.
pub fn read_i8(bytes: &[u8], _endianness: Endianness) -> i8 {
  bytes[0] as i8
}

/// Reads an `i16` from a raw byte slice using the configured endianness.
pub fn read_i16(bytes: &[u8], endianness: Endianness) -> i16 {
  let bytes = [bytes[0], bytes[1]];
  match endianness {
    Endianness::Little => i16::from_le_bytes(bytes),
    Endianness::Big => i16::from_be_bytes(bytes),
  }
}

/// Reads an `i32` from a raw byte slice using the configured endianness.
pub fn read_i32(bytes: &[u8], endianness: Endianness) -> i32 {
  let bytes = [
    bytes[0],
    bytes[1],
    bytes[2],
    bytes[3],
  ];
  match endianness {
    Endianness::Little => i32::from_le_bytes(bytes),
    Endianness::Big => i32::from_be_bytes(bytes),
  }
}

/// Reads an `f32` from a raw byte slice using the configured endianness.
pub fn read_f32(bytes: &[u8], endianness: Endianness) -> f32 {
  let bytes = [
    bytes[0],
    bytes[1],
    bytes[2],
    bytes[3],
  ];
  match endianness {
    Endianness::Little => f32::from_le_bytes(bytes),
    Endianness::Big => f32::from_be_bytes(bytes),
  }
}
