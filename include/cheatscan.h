#ifndef CHEATSCAN_H
#define CHEATSCAN_H

#include <stddef.h>
#include <stdint.h>
#include "./cheatscan_types.h"

#ifdef __cplusplus
extern "C" {
#endif

Scanner *cheatscan_new_from_unknown(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_u8(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t cmp,
  uint8_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_u16(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t cmp,
  uint16_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_u32(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t cmp,
  uint32_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_i8(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t cmp,
  int8_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_i16(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t cmp,
  int16_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_i32(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t cmp,
  int32_t value,
  uint8_t *out_error);

Scanner *cheatscan_new_from_known_f32(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t cmp,
  float value,
  uint8_t *out_error);

uint8_t cheatscan_scan_previous(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp);

uint8_t cheatscan_scan_u8(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp,
  uint8_t value);

uint8_t cheatscan_scan_u16(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp,
  uint16_t value);

uint8_t cheatscan_scan_u32(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp,
  uint32_t value);

uint8_t cheatscan_scan_i8(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp,
  int8_t value);

uint8_t cheatscan_scan_i16(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp,
  int16_t value);

uint8_t cheatscan_scan_i32(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp,
  int32_t value);

uint8_t cheatscan_scan_f32(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp,
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
