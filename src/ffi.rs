use crate::Alignment;
use crate::ComparisonType;
use crate::Configuration;
use crate::Endianness;
use crate::ScanError;
use crate::ScanValue;
use crate::Scanner;
use crate::ValueType;
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

// pub fn cheatscan::Scanner::new_from_unknown(config: cheatscan::Configuration, initial_block: &[u8]) -> core::result::Result<Self, cheatscan::ScanError>
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

// pub fn cheatscan::Scanner::new_from_known(config: cheatscan::Configuration, initial_block: &[u8], cmp: cheatscan::ComparisonType, value: cheatscan::ScanValue) -> core::result::Result<Self, cheatscan::ScanError>
macro_rules! define_new_from_known_fn {
  ($fn_name:ident, $value_ty:ty, $variant:ident) => {
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

// pub fn cheatscan::Scanner::scan_previous(&mut self, next_block: &[u8], cmp: cheatscan::ComparisonType) -> core::result::Result<(), cheatscan::ScanError>
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

// pub fn cheatscan::Scanner::scan_exact_bytes(&mut self, next_block: &[u8], cmp: cheatscan::ComparisonType, value: &[u8]) -> core::result::Result<(), cheatscan::ScanError>
macro_rules! define_scan_fn {
  ($fn_name:ident, $value_ty:ty, $variant:ident) => {
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

// pub fn cheatscan::Scanner::count(&self) -> usize
#[unsafe(no_mangle)]
pub extern "C" fn cheatscan_count(scanner: *mut Scanner) -> u32 {
  check_not_null!(scanner, 0);

  let scanner = unsafe { &*scanner };
  scanner.count() as u32
}

// pub fn cheatscan::Scanner::results(&self) -> impl core::iter::traits::iterator::Iterator<Item = u32> + '_
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

#[unsafe(no_mangle)]
pub extern "C" fn cheatscan_free(scanner: *mut Scanner) {
  if scanner.is_null() {
    return;
  }

  unsafe { drop(Box::from_raw(scanner)) }
}
