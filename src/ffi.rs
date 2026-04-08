use crate::scanner::Scanner;
use crate::scanner::{
  Alignment, ComparisonType, Configuration, Endianness, ScanError, ScanValue, ValueType,
};

use core::slice;

macro_rules! check_not_null {
  ($ptr:expr, $ret:expr) => {
    if $ptr.is_null() {
      return $ret;
    }
  };
}

macro_rules! ffi_try {
  ($ptr:expr, $ret:expr) => {
    match $ptr {
      Ok(v) => v,
      Err(_) => return $ret,
    }
  };
}

macro_rules! ffi_result {
  ($expr:expr) => {
    match $expr {
      Ok(()) => 0,
      Err(err) => err as u8,
    }
  };
}

/// Creates a new scanner from an initial RAM block without applying an initial filter.
///
/// This is the FFI entry point for the Rust method `Scanner::new_from_unknown`. It is the
/// constructor to use for classic "unknown initial value" workflows.
///
/// The `value_type`, `endianness`, and `alignment` parameters are raw `u8` discriminants matching
/// the corresponding C ABI enums. `base_address` is added to every result reported later by
/// `cheatscan_write_results`.
///
/// On success, returns an owning pointer that must eventually be released with `cheatscan_free`.
/// On failure, returns null and, when `out_error` is non-null, writes the matching `ScanError`
/// discriminant.
///
/// Null `initial_block_ptr` is treated as failure and returns null.
#[unsafe(no_mangle)]
pub extern "C" fn cheatscan_new_from_unknown(
  value_type: u8,
  endianness: u8,
  alignment: u8,
  base_address: u32,
  initial_block_ptr: *const u8,
  initial_block_len: usize,
  out_error: *mut u8,
) -> *mut Scanner {
  check_not_null!(initial_block_ptr, core::ptr::null_mut());
  let cfg_value_type = ffi_try!(ValueType::try_from(value_type), core::ptr::null_mut());
  let cfg_endienness = ffi_try!(Endianness::try_from(endianness), core::ptr::null_mut());
  let cfg_alignment = ffi_try!(Alignment::try_from(alignment), core::ptr::null_mut());
  let config = Configuration {
    value_type: cfg_value_type,
    endianness: cfg_endienness,
    alignment: cfg_alignment,
    base_address,
  };

  let result = unsafe {
    let initial_block = slice::from_raw_parts(initial_block_ptr, initial_block_len);
    Scanner::new_from_unknown(config, initial_block)
  };

  match result {
    Ok(scanner) => {
      if !out_error.is_null() {
        unsafe { *out_error = 0 };
      }
      Box::into_raw(Box::new(scanner))
    }
    Err(err) => {
      if !out_error.is_null() {
        unsafe { *out_error = err as u8 };
      }
      core::ptr::null_mut()
    }
  }
}

// pub fn cheatscan::Scanner::new_from_known(config: cheatscan::Configuration, initial_block: &[u8], cmp: cheatscan::ComparisonType, value: cheatscan::ScanValue) -> Result<Self, ScanError>
macro_rules! define_new_from_known_fn {
  ($fn_name:ident, $value_ty:ty, $variant:ident) => {
    #[doc = "Creates a new scanner and immediately performs the first exact-value scan."]
    #[doc = ""]
    #[doc = "This is the FFI equivalent of `Scanner::new_from_known` for one concrete value type."]
    #[doc = "The `value` parameter must match the function suffix and the `value_type` ABI"]
    #[doc = "discriminant supplied by the caller."]
    #[doc = ""]
    #[doc = "On success, returns an owning scanner pointer that must be freed with"]
    #[doc = "`cheatscan_free`. On failure, returns null and optionally writes a `ScanError`"]
    #[doc = "code to `out_error`."]
    #[doc = ""]
    #[doc = "Passing `initial_block_ptr = NULL` returns null immediately."]
    #[unsafe(no_mangle)]
    pub extern "C" fn $fn_name(
      value_type: u8,
      endianness: u8,
      alignment: u8,
      base_address: u32,
      initial_block_ptr: *const u8,
      initial_block_len: usize,
      cmp: u8,
      value: $value_ty,
      out_error: *mut u8,
    ) -> *mut Scanner {
      check_not_null!(initial_block_ptr, core::ptr::null_mut());
      let cfg_value_type = ffi_try!(ValueType::try_from(value_type), core::ptr::null_mut());
      let cfg_endienness = ffi_try!(Endianness::try_from(endianness), core::ptr::null_mut());
      let cfg_alignment = ffi_try!(Alignment::try_from(alignment), core::ptr::null_mut());
      let config = Configuration {
        value_type: cfg_value_type,
        endianness: cfg_endienness,
        alignment: cfg_alignment,
        base_address,
      };
      let cmp = ffi_try!(ComparisonType::try_from(cmp), core::ptr::null_mut());

      let result = unsafe {
        let initial_block = slice::from_raw_parts(initial_block_ptr, initial_block_len);
        Scanner::new_from_known(config, initial_block, cmp, ScanValue::$variant(value))
      };

      match result {
        Ok(scanner) => {
          if !out_error.is_null() {
            unsafe { *out_error = 0 };
          }
          Box::into_raw(Box::new(scanner))
        }
        Err(err) => {
          if !out_error.is_null() {
            unsafe { *out_error = err as u8 };
          }
          core::ptr::null_mut()
        }
      }
    }
  };
}
define_new_from_known_fn!(cheatscan_new_from_known_u8, u8, U8);
define_new_from_known_fn!(cheatscan_new_from_known_u16, u16, U16);
define_new_from_known_fn!(cheatscan_new_from_known_u32, u32, U32);
define_new_from_known_fn!(cheatscan_new_from_known_i8, i8, I8);
define_new_from_known_fn!(cheatscan_new_from_known_i16, i16, I16);
define_new_from_known_fn!(cheatscan_new_from_known_i32, i32, I32);
define_new_from_known_fn!(cheatscan_new_from_known_f32, f32, F32);

/// Filters the scanner against a new RAM snapshot using a previous-value comparison.
///
/// This is the FFI form of `Scanner::scan(..., ScanValue::PreviousValue)`. Each candidate value
/// read from `next_block_ptr` is compared against the value stored in the scanner's previous RAM
/// block at the same offset.
///
/// Returns `0` on success. Any non-zero return value is a `ScanError` discriminant.
///
/// A null `scanner` or `next_block_ptr` yields `ScanError::NullPointer`. An invalid comparison
/// discriminant yields `ScanError::TypeMismatch`.
#[unsafe(no_mangle)]
pub extern "C" fn cheatscan_scan_previous(
  scanner: *mut Scanner,
  next_block_ptr: *const u8,
  next_block_len: usize,
  cmp: u8,
) -> u8 {
  check_not_null!(scanner, ScanError::NullPointer as u8);
  check_not_null!(next_block_ptr, ScanError::NullPointer as u8);
  let cmp = ffi_try!(ComparisonType::try_from(cmp), ScanError::TypeMismatch as u8);

  let result = unsafe {
    let scanner = &mut *scanner;
    let next_block = slice::from_raw_parts(next_block_ptr, next_block_len);

    scanner.scan(next_block, cmp, ScanValue::PreviousValue)
  };

  ffi_result!(result)
}

macro_rules! define_scan_fn {
  ($fn_name:ident, $value_ty:ty, $variant:ident) => {
    #[doc = "Filters the scanner against a new RAM snapshot using an exact value."]
    #[doc = ""]
    #[doc = "This is the FFI equivalent of `Scanner::scan` for one concrete value type. The"]
    #[doc = "function suffix determines the ABI type of `value`, and that type must agree with"]
    #[doc = "the runtime `value_type` configured in the scanner."]
    #[doc = ""]
    #[doc = "Returns `0` on success. Any non-zero return value is a `ScanError` discriminant."]
    #[doc = ""]
    #[doc = "A null `scanner` or `next_block_ptr` yields `ScanError::NullPointer`. An invalid"]
    #[doc = "comparison discriminant yields `ScanError::TypeMismatch`."]
    #[unsafe(no_mangle)]
    pub extern "C" fn $fn_name(
      scanner: *mut Scanner,
      next_block_ptr: *const u8,
      next_block_len: usize,
      cmp: u8,
      value: $value_ty,
    ) -> u8 {
      check_not_null!(scanner, ScanError::NullPointer as u8);
      check_not_null!(next_block_ptr, ScanError::NullPointer as u8);
      let cmp = ffi_try!(ComparisonType::try_from(cmp), ScanError::TypeMismatch as u8);

      let result = unsafe {
        let scanner = &mut *scanner;
        let next_block = slice::from_raw_parts(next_block_ptr, next_block_len);

        scanner.scan(next_block, cmp, ScanValue::$variant(value))
      };

      ffi_result!(result)
    }
  };
}
define_scan_fn!(cheatscan_scan_u8, u8, U8);
define_scan_fn!(cheatscan_scan_u16, u16, U16);
define_scan_fn!(cheatscan_scan_u32, u32, U32);
define_scan_fn!(cheatscan_scan_i8, i8, I8);
define_scan_fn!(cheatscan_scan_i16, i16, I16);
define_scan_fn!(cheatscan_scan_i32, i32, I32);
define_scan_fn!(cheatscan_scan_f32, f32, F32);

/// Refines the scanner against its current stored RAM block using an exact value.
///
/// This corresponds to `Scanner::scan_again` and does not accept a new RAM snapshot. It is meant
/// for chaining additional exact-value filters on the current block after one or more prior scans.
///
/// Returns `0` on success. Any non-zero return value is a `ScanError` discriminant.
///
/// A null `scanner` yields `ScanError::NullPointer`. An invalid comparison discriminant yields
/// `ScanError::TypeMismatch`.
macro_rules! define_scan_again_fn {
  ($fn_name:ident, $value_ty:ty, $variant:ident) => {
    #[doc = "Refines the scanner against its current stored RAM block using an exact value."]
    #[doc = ""]
    #[doc = "This is the typed FFI form of `Scanner::scan_again`. The function suffix determines"]
    #[doc = "the ABI type of `value`, and that type must match the scanner's configured"]
    #[doc = "`value_type`."]
    #[doc = ""]
    #[doc = "Returns `0` on success. Any non-zero return value is a `ScanError` discriminant."]
    #[doc = ""]
    #[doc = "A null `scanner` yields `ScanError::NullPointer`. An invalid comparison"]
    #[doc = "discriminant yields `ScanError::TypeMismatch`."]
    #[unsafe(no_mangle)]
    pub extern "C" fn $fn_name(scanner: *mut Scanner, cmp: u8, value: $value_ty) -> u8 {
      check_not_null!(scanner, ScanError::NullPointer as u8);
      let cmp = ffi_try!(ComparisonType::try_from(cmp), ScanError::TypeMismatch as u8);

      let result = unsafe {
        let scanner = &mut *scanner;

        scanner.scan_again(cmp, ScanValue::$variant(value))
      };

      ffi_result!(result)
    }
  };
}
define_scan_again_fn!(cheatscan_scan_again_u8, u8, U8);
define_scan_again_fn!(cheatscan_scan_again_u16, u16, U16);
define_scan_again_fn!(cheatscan_scan_again_u32, u32, U32);
define_scan_again_fn!(cheatscan_scan_again_i8, i8, I8);
define_scan_again_fn!(cheatscan_scan_again_i16, i16, I16);
define_scan_again_fn!(cheatscan_scan_again_i32, i32, I32);
define_scan_again_fn!(cheatscan_scan_again_f32, f32, F32);

/// Returns the current candidate count for a scanner.
///
/// This matches `Scanner::count`. Before the first successful filtering pass, the returned value is
/// the size of the implicit search space; after filtering starts, it is the number of materialized
/// surviving candidates.
///
/// A null `scanner` returns `0`.
#[unsafe(no_mangle)]
pub extern "C" fn cheatscan_count(scanner: *mut Scanner) -> u32 {
  check_not_null!(scanner, 0);

  let scanner = unsafe { &*scanner };
  scanner.count() as u32
}

/// Writes materialized result addresses into a caller-provided output buffer.
///
/// This is the FFI equivalent of iterating over `Scanner::results()` and copying the addresses
/// into `out_results_ptr`. The `offset` parameter allows paginating through the current result set:
/// the function skips the first `offset` materialized results, then writes at most
/// `out_results_len` addresses.
///
/// Returned addresses already include the scanner's configured base address.
///
/// The return value is the number of addresses actually written. If either `scanner` or
/// `out_results_ptr` is null, the function returns `0`.
#[unsafe(no_mangle)]
pub extern "C" fn cheatscan_write_results(
  scanner: *mut Scanner,
  out_results_ptr: *mut u32,
  out_results_len: usize,
  offset: usize,
) -> usize {
  if scanner.is_null() || out_results_ptr.is_null() {
    return 0;
  }

  let scanner = unsafe { &*scanner };
  let out_results = unsafe { slice::from_raw_parts_mut(out_results_ptr, out_results_len) };

  let mut written = 0;

  for (i, value) in scanner
    .results()
    .skip(offset)
    .take(out_results_len)
    .enumerate()
  {
    out_results[i] = value;
    written += 1;
  }

  written
}

/// Releases a scanner previously allocated by one of the `cheatscan_new_*` constructors.
///
/// Passing null is allowed and does nothing.
#[unsafe(no_mangle)]
pub extern "C" fn cheatscan_free(scanner: *mut Scanner) {
  if scanner.is_null() {
    return;
  }

  unsafe { drop(Box::from_raw(scanner)) }
}
