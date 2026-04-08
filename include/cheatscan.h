#ifndef CHEATSCAN_H
#define CHEATSCAN_H

#include <stddef.h>
#include <stdint.h>
#include "./cheatscan_types.h"

#ifdef __cplusplus
extern "C" {
#endif

Scanner *cheatscan_new_from_unknown(
  CheatscanValueType value_type,
  CheatscanEndianness endiannness,
  CheatscanAlignment alignment,
  uint32_t base_address,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_u8(
  CheatscanValueType value_type,
  CheatscanEndianness endiannness,
  CheatscanAlignment alignment,
  uint32_t base_address,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  CheatscanComparisonType cmp,
  uint8_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_u16(
  CheatscanValueType value_type,
  CheatscanEndianness endiannness,
  CheatscanAlignment alignment,
  uint32_t base_address,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  CheatscanComparisonType cmp,
  uint16_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_u32(
  CheatscanValueType value_type,
  CheatscanEndianness endiannness,
  CheatscanAlignment alignment,
  uint32_t base_address,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  CheatscanComparisonType cmp,
  uint32_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_i8(
  CheatscanValueType value_type,
  CheatscanEndianness endiannness,
  CheatscanAlignment alignment,
  uint32_t base_address,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  CheatscanComparisonType cmp,
  int8_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_i16(
  CheatscanValueType value_type,
  CheatscanEndianness endiannness,
  CheatscanAlignment alignment,
  uint32_t base_address,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  CheatscanComparisonType cmp,
  int16_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_i32(
  CheatscanValueType value_type,
  CheatscanEndianness endiannness,
  CheatscanAlignment alignment,
  uint32_t base_address,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  CheatscanComparisonType cmp,
  int32_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_f32(
  CheatscanValueType value_type,
  CheatscanEndianness endiannness,
  CheatscanAlignment alignment,
  uint32_t base_address,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  CheatscanComparisonType cmp,
  float value,
  uint8_t *out_error);

uint8_t cheatscan_scan_previous(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  CheatscanComparisonType cmp);

uint8_t cheatscan_scan_u8(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  CheatscanComparisonType cmp,
  uint8_t value);

uint8_t cheatscan_scan_u16(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  CheatscanComparisonType cmp,
  uint16_t value);

uint8_t cheatscan_scan_u32(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  CheatscanComparisonType cmp,
  uint32_t value);

uint8_t cheatscan_scan_i8(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  CheatscanComparisonType cmp,
  int8_t value);

uint8_t cheatscan_scan_i16(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  CheatscanComparisonType cmp,
  int16_t value);

uint8_t cheatscan_scan_i32(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  CheatscanComparisonType cmp,
  int32_t value);

uint8_t cheatscan_scan_f32(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  CheatscanComparisonType cmp,
  float value);

uint8_t cheatscan_scan_again_u8(
  Scanner *scanner,
  CheatscanComparisonType cmp,
  uint8_t value);

uint8_t cheatscan_scan_again_u16(
  Scanner *scanner,
  CheatscanComparisonType cmp,
  uint16_t value);

uint8_t cheatscan_scan_again_u32(
  Scanner *scanner,
  CheatscanComparisonType cmp,
  uint32_t value);

uint8_t cheatscan_scan_again_i8(
  Scanner *scanner,
  CheatscanComparisonType cmp,
  int8_t value);

uint8_t cheatscan_scan_again_i16(
  Scanner *scanner,
  CheatscanComparisonType cmp,
  int16_t value);

uint8_t cheatscan_scan_again_i32(
  Scanner *scanner,
  CheatscanComparisonType cmp,
  int32_t value);

uint8_t cheatscan_scan_again_f32(
  Scanner *scanner,
  CheatscanComparisonType cmp,
  float value);

uint32_t cheatscan_count(
  Scanner *scanner);

size_t cheatscan_write_results(
  Scanner *scanner,
  uint32_t *out_results_ptr,
  size_t out_results_len,
  size_t offset);

void cheatscan_free(
  Scanner *scanner);

#ifdef __cplusplus
}
#endif

#endif
