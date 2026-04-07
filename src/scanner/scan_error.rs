/// Errors that can occur while constructing or scanning a [`Scanner`](crate::Scanner).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScanError {
  /// The provided value does not match the configured [`ValueType`](crate::ValueType).
  ///
  /// Triggered when a scan is requested with a [`ScanValue`](crate::ScanValue) whose variant is
  /// incompatible with the scanner configuration.
  TypeMismatch = 1,

  /// The provided bytes are not long enough to be converted into  [`ValueType`](crate::ValueType).
  ///
  /// Triggered when a scan is requested with a byte array whose width is incompatible with the
  /// scanner configuration.
  InvalidValueLength = 2,

  /// A computed result address could not fit in `u32`.
  ///
  /// Reserved for address computations that would overflow the public address space exposed by
  /// the scanner.
  AddressOverflow = 3,

  /// A null pointer was passed through an FFI boundary where a valid RAM block was required.
  ///
  /// This error is meant for higher-level FFI adapters built on top of the Rust core.
  NullPointer = 4,

  /// The provided RAM block length does not match the scanner's stored baseline length.
  ///
  /// Triggered when [`Scanner::scan`](crate::Scanner::scan) receives a `next_block` with a
  /// different length than the block used to initialize the scanner.
  InvalidRamBlockLength = 5,

  /// `PreviousValue` was used where an immediate exact-value initialization was required.
  ///
  /// Triggered when [`Scanner::new_from_known`](crate::Scanner::new_from_known) is called with
  /// [`ScanValue::PreviousValue`](crate::ScanValue::PreviousValue).
  InitialScanValueRequired = 6,

  /// The provided RAM block is too small to hold even one value of the configured type.
  ///
  /// Triggered during scanner initialization when `ram_block.len() < value_type.width()`.
  RamBlockTooSmall = 7,
}
