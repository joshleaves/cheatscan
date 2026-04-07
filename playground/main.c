#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include "../include/cheatscan.h"

int main(void) {
  uint8_t initial_block[] = { 10, 20, 30, 40 };
  uint8_t next_block[] = { 10, 25, 30, 35 };

  uint8_t error = 0;
  Scanner *scanner = cheatscan_new_from_unknown(
    CHEATSCAN_VALUE_U8,            // .value_type
    CHEATSCAN_ENDIAN_LITTLE,       // .endianness
    CHEATSCAN_ALIGNMENT_UNALIGNED, // .alignment
    0, // .base_address
    initial_block,
    sizeof(initial_block),
    &error
  );

  if (!scanner) {
    printf("new failed: %u\n", error);
    return 1;
  }

  error = cheatscan_scan_previous(
    scanner,
    next_block,
    sizeof(next_block),
    CHEATSCAN_CMP_GT
  );

  if (error != 0) {
    printf("scan failed: %u\n", error);
    cheatscan_free(scanner);
    return 1;
  }

  uint32_t count = cheatscan_count(scanner);
  printf("count = %u\n", count);

  uint32_t results[16];
  size_t written = cheatscan_write_results(
    scanner,
    results,
    16,
    0
  );

  for (size_t i = 0; i < written; i++) {
    printf("result[%zu] = %u\n", i, results[i]);
  }

  cheatscan_free(scanner);
  return 0;
}