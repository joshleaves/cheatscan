#ifndef CHEATSCAN_TYPES_H
#define CHEATSCAN_TYPES_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Primitive value kinds supported by the scanner.
 */
enum ValueType {
  CHEATSCAN_VALUE_U8 = 0,
  CHEATSCAN_VALUE_U16 = 1,
  CHEATSCAN_VALUE_U32 = 2,
  CHEATSCAN_VALUE_I8 = 3,
  CHEATSCAN_VALUE_I16 = 4,
  CHEATSCAN_VALUE_I32 = 5,
  CHEATSCAN_VALUE_F32 = 6,
};
typedef uint8_t ValueType;

/**
 * Byte order used when reading multi-byte values from a RAM block.
 */
enum Endianness {
  CHEATSCAN_ENDIAN_LITTLE = 0,
  CHEATSCAN_ENDIAN_BIG = 1,
};
typedef uint8_t Endianness;

/**
 * Address stepping strategy used when enumerating scan candidates.
 */
enum Alignment {
  CHEATSCAN_ALIGNMENT_UNALIGNED = 0,
  CHEATSCAN_ALIGNMENT_ALIGNED = 1,
};
typedef uint8_t Alignment;

/**
 * Comparison operator applied during a scan.
 */
enum ComparisonType {
  CHEATSCAN_CMP_EQ = 0,
  CHEATSCAN_CMP_NE = 1,
  CHEATSCAN_CMP_LT = 2,
  CHEATSCAN_CMP_LE = 3,
  CHEATSCAN_CMP_GT = 4,
  CHEATSCAN_CMP_GE = 5,
};
typedef uint8_t ComparisonType;

/**
 * Errors that can occur while constructing or scanning a [`Scanner`](crate::Scanner).
 */
enum ScanError {
  /**
   * The provided value does not match the configured [`ValueType`](crate::ValueType).
   *
   * Triggered when a scan is requested with a [`ScanValue`](crate::ScanValue) whose variant is
   * incompatible with the scanner configuration.
   */
  TypeMismatch = 1,
  /**
   * The provided bytes are not long enough to be converted into  [`ValueType`](crate::ValueType).
   *
   * Triggered when a scan is requested with a byte array whose width is incompatible with the
   * scanner configuration.
   */
  InvalidValueLength = 2,
  /**
   * A computed result address could not fit in `u32`.
   *
   * Reserved for address computations that would overflow the public address space exposed by
   * the scanner.
   */
  AddressOverflow = 3,
  /**
   * A null pointer was passed through an FFI boundary where a valid RAM block was required.
   *
   * This error is meant for higher-level FFI adapters built on top of the Rust core.
   */
  NullPointer = 4,
  /**
   * The provided RAM block length does not match the scanner's stored baseline length.
   *
   * Triggered when [`Scanner::scan`](crate::Scanner::scan) receives a `next_block` with a
   * different length than the block used to initialize the scanner.
   */
  InvalidRamBlockLength = 5,
  /**
   * `PreviousValue` was used where an immediate exact-value initialization was required.
   *
   * Triggered when [`Scanner::new_from_known`](crate::Scanner::new_from_known) is called with
   * [`ScanValue::PreviousValue`](crate::ScanValue::PreviousValue).
   */
  InitialScanValueRequired = 6,
  /**
   * The provided RAM block is too small to hold even one value of the configured type.
   *
   * Triggered during scanner initialization when `ram_block.len() < value_type.width()`.
   */
  RamBlockTooSmall = 7,
};
typedef uint8_t ScanError;

/**
 * Stateful memory scanner that keeps the previous RAM block and the current candidate set.
 */
typedef struct Scanner Scanner;

/**
 * Immutable scanner configuration shared across all scans.
 */
typedef struct Configuration {
  ValueType value_type;
  Endianness endianness;
  Alignment alignment;
  uint32_t base_address;
} Configuration;


#endif  /* CHEATSCAN_TYPES_H */
