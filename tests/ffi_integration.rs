#![cfg(feature = "ffi")]

use cheatscan::ffi::{
  cheatscan_count, cheatscan_free, cheatscan_new_from_known_u16, cheatscan_new_from_unknown,
  cheatscan_scan_f32, cheatscan_scan_u16, cheatscan_write_results,
};
use cheatscan::{Alignment, ComparisonType, Endianness, ScanError, ValueType};

const VALUE_U16: u8 = ValueType::U16 as u8;
const VALUE_F32: u8 = ValueType::F32 as u8;

const ENDIAN_LITTLE: u8 = Endianness::Little as u8;
const ENDIAN_BIG: u8 = Endianness::Big as u8;

const ALIGN_UNALIGNED: u8 = Alignment::Unaligned as u8;
const ALIGN_ALIGNED: u8 = Alignment::Aligned as u8;

const CMP_EQ: u8 = ComparisonType::Eq as u8;

const ERR_TYPE_MISMATCH: u8 = ScanError::TypeMismatch as u8;
const ERR_ADDRESS_OVERFLOW: u8 = ScanError::AddressOverflow as u8;
const ERR_NULL_POINTER: u8 = ScanError::NullPointer as u8;

fn collect_results(scanner: *mut cheatscan::Scanner) -> Vec<u32> {
  let count = cheatscan_count(scanner) as usize;
  let mut out = vec![0_u32; count];
  let written = cheatscan_write_results(scanner, out.as_mut_ptr(), out.len(), 0);
  out.truncate(written);
  out
}

#[test]
fn constructors_write_out_error_on_early_failures() {
  let mut err = 0_u8;
  let scanner = cheatscan_new_from_unknown(
    VALUE_U16,
    ENDIAN_LITTLE,
    ALIGN_ALIGNED,
    0,
    core::ptr::null(),
    4,
    &mut err,
  );
  assert!(scanner.is_null());
  assert_eq!(err, ERR_NULL_POINTER);

  let block = [0_u8, 1, 2, 3];
  let mut err = 0_u8;
  let scanner = cheatscan_new_from_unknown(
    255,
    ENDIAN_LITTLE,
    ALIGN_ALIGNED,
    0,
    block.as_ptr(),
    block.len(),
    &mut err,
  );
  assert!(scanner.is_null());
  assert_eq!(err, ERR_TYPE_MISMATCH);
}

#[test]
fn constructor_reports_address_overflow() {
  let block = [1_u8, 2_u8];
  let mut err = 0_u8;
  let scanner = cheatscan_new_from_unknown(
    0,
    ENDIAN_LITTLE,
    ALIGN_UNALIGNED,
    u32::MAX,
    block.as_ptr(),
    block.len(),
    &mut err,
  );
  assert!(scanner.is_null());
  assert_eq!(err, ERR_ADDRESS_OVERFLOW);
}

#[test]
fn known_u16_respects_endianness() {
  let block = [
    0x12_u8,
    0x34_u8,
    0x34_u8,
    0x12_u8,
  ];

  let mut err_le = 255_u8;
  let scanner_le = cheatscan_new_from_known_u16(
    VALUE_U16,
    ENDIAN_LITTLE,
    ALIGN_ALIGNED,
    0,
    block.as_ptr(),
    block.len(),
    CMP_EQ,
    0x1234,
    &mut err_le,
  );
  assert!(!scanner_le.is_null());
  assert_eq!(err_le, 0);
  assert_eq!(collect_results(scanner_le), vec![2]);
  cheatscan_free(scanner_le);

  let mut err_be = 255_u8;
  let scanner_be = cheatscan_new_from_known_u16(
    VALUE_U16,
    ENDIAN_BIG,
    ALIGN_ALIGNED,
    0,
    block.as_ptr(),
    block.len(),
    CMP_EQ,
    0x1234,
    &mut err_be,
  );
  assert!(!scanner_be.is_null());
  assert_eq!(err_be, 0);
  assert_eq!(collect_results(scanner_be), vec![0]);
  cheatscan_free(scanner_be);
}

#[test]
fn unaligned_u16_scan_finds_mid_byte_candidate() {
  let initial = [1_u8, 2_u8, 3_u8];
  let mut err = 255_u8;
  let scanner = cheatscan_new_from_unknown(
    VALUE_U16,
    ENDIAN_LITTLE,
    ALIGN_UNALIGNED,
    0,
    initial.as_ptr(),
    initial.len(),
    &mut err,
  );
  assert!(!scanner.is_null());
  assert_eq!(err, 0);

  let scan_err = cheatscan_scan_u16(scanner, initial.as_ptr(), initial.len(), CMP_EQ, 0x0302);
  assert_eq!(scan_err, 0);
  assert_eq!(collect_results(scanner), vec![1]);
  cheatscan_free(scanner);
}

#[test]
fn float_edge_cases_follow_ieee_comparisons() {
  let mut block = Vec::new();
  block.extend_from_slice(&0.0_f32.to_le_bytes());
  block.extend_from_slice(&(-0.0_f32).to_le_bytes());
  block.extend_from_slice(&f32::NAN.to_le_bytes());

  let mut err = 255_u8;
  let scanner = cheatscan_new_from_unknown(
    VALUE_F32,
    ENDIAN_LITTLE,
    ALIGN_ALIGNED,
    0,
    block.as_ptr(),
    block.len(),
    &mut err,
  );
  assert!(!scanner.is_null());
  assert_eq!(err, 0);

  let scan_eq_negative_zero =
    cheatscan_scan_f32(scanner, block.as_ptr(), block.len(), CMP_EQ, -0.0);
  assert_eq!(scan_eq_negative_zero, 0);
  assert_eq!(collect_results(scanner), vec![0, 4]);
  cheatscan_free(scanner);

  let mut err = 255_u8;
  let scanner = cheatscan_new_from_unknown(
    VALUE_F32,
    ENDIAN_LITTLE,
    ALIGN_ALIGNED,
    0,
    block.as_ptr(),
    block.len(),
    &mut err,
  );
  assert!(!scanner.is_null());
  assert_eq!(err, 0);

  let scan_eq_nan = cheatscan_scan_f32(scanner, block.as_ptr(), block.len(), CMP_EQ, f32::NAN);
  assert_eq!(scan_eq_nan, 0);
  assert_eq!(collect_results(scanner), Vec::<u32>::new());
  cheatscan_free(scanner);
}
