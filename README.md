# cheatscan

`cheatscan` is a small Rust library for memory scanning.

It is designed around the classic cheat-search workflow:

- start from an initial RAM block
- scan for an exact value, or compare against the previous value
- keep narrowing the candidate addresses over multiple scans

The crate currently exposes:

- a Rust API
- a C-compatible FFI with generated headers in [`include/`](/Users/red/Projects/cheatscan/include)

## Features

- `u8`, `u16`, `u32`
- `i8`, `i16`, `i32`
- `f32`
- little-endian and big-endian reads
- aligned and unaligned scans
- exact-value scans
- previous-value scans
- incremental narrowing of candidate addresses

## Project Layout

The scanner domain lives under [`src/scanner/`](/Users/red/Projects/cheatscan/src/scanner):

- [`mod.rs`](/Users/red/Projects/cheatscan/src/scanner/mod.rs): public facade
- [`scanner.rs`](/Users/red/Projects/cheatscan/src/scanner/scanner.rs): `Scanner`
- [`comparison.rs`](/Users/red/Projects/cheatscan/src/scanner/comparison.rs): comparison operators
- [`scan_error.rs`](/Users/red/Projects/cheatscan/src/scanner/scan_error.rs): scanner errors
- [`value_reader.rs`](/Users/red/Projects/cheatscan/src/scanner/value_reader.rs): typed byte readers
- [`value_type.rs`](/Users/red/Projects/cheatscan/src/scanner/value_type.rs): supported primitive types

The FFI layer lives in [`src/ffi.rs`](/Users/red/Projects/cheatscan/src/ffi.rs).

## Rust API

Core public types are exported from [`src/lib.rs`](/Users/red/Projects/cheatscan/src/lib.rs):

- `Scanner`
- `ScanValue`
- `ValueType`
- `ComparisonType`
- `ScanError`
- `Configuration`
- `Endianness`
- `Alignment`

### Configuration

```rust
use cheatscan::{Alignment, Configuration, Endianness, ValueType};

let config = Configuration {
  value_type: ValueType::U16,
  endianness: Endianness::Little,
  alignment: Alignment::Aligned,
  base_address: 0x8000,
};
```

### Unknown Initial Value

Use `Scanner::new_from_unknown(...)` when the first snapshot is only used as the baseline.

```rust
use cheatscan::{ComparisonType, Configuration, ScanValue, Scanner};

let initial_block = [10_u8, 20, 30, 40];
let next_block = [10_u8, 19, 31, 40];

let mut scanner = Scanner::new_from_unknown(config, &initial_block)?;
scanner.scan(&next_block, ComparisonType::Gt, ScanValue::PreviousValue)?;

let results: Vec<u32> = scanner.results().collect();
```

### Known Initial Value

Use `Scanner::new_from_known(...)` when the first snapshot should already be filtered against a known value.

```rust
use cheatscan::{ComparisonType, Configuration, ScanValue, Scanner};

let initial_block = [1_u8, 0, 2, 0, 1, 0];

let scanner = Scanner::new_from_known(
  config,
  &initial_block,
  ComparisonType::Eq,
  ScanValue::U16(1),
)?;

let results: Vec<u32> = scanner.results().collect();
```

### Follow-up Scans

After initialization, `scan(...)` supports both modes:

- exact value: `ScanValue::U8(...)`, `ScanValue::I16(...)`, etc.
- previous value: `ScanValue::PreviousValue`

Example:

```rust
scanner.scan(&next_block_a, ComparisonType::Gt, ScanValue::PreviousValue)?;
scanner.scan(&next_block_b, ComparisonType::Eq, ScanValue::U8(54))?;
scanner.scan(&next_block_c, ComparisonType::Lt, ScanValue::PreviousValue)?;
```

### Results

- `count()` returns the current number of candidates
- `results()` returns materialized result addresses, already offset by `base_address`

Before the first filtering pass:

- `count()` reports the implicit candidate count
- `results()` is empty because no concrete result set has been materialized yet

### Errors

Relevant scanner errors include:

- `TypeMismatch`: scan value does not match `value_type`
- `InvalidRamBlockLength`: next RAM block length differs from the initial one
- `InitialScanValueRequired`: `new_from_known(...)` was called with `ScanValue::PreviousValue`
- `RamBlockTooSmall`: the RAM block cannot contain even one value of the configured type

See [`src/scanner/scan_error.rs`](/Users/red/Projects/cheatscan/src/scanner/scan_error.rs) for details.

## C / FFI API

The FFI surface is implemented in [`src/ffi.rs`](/Users/red/Projects/cheatscan/src/ffi.rs) and declared in:

- [`include/cheatscan.h`](/Users/red/Projects/cheatscan/include/cheatscan.h)
- [`include/cheatscan_types.h`](/Users/red/Projects/cheatscan/include/cheatscan_types.h)

### Construction

Unknown initial value:

```c
Scanner *cheatscan_new_from_unknown(
  Configuration config,
  const uint8_t *initial_block_ptr,
  size_t initial_block_len,
  uint8_t *out_error);
```

Known initial value is exposed as typed constructors:

- `cheatscan_new_from_known_u8`
- `cheatscan_new_from_known_u16`
- `cheatscan_new_from_known_u32`
- `cheatscan_new_from_known_i8`
- `cheatscan_new_from_known_i16`
- `cheatscan_new_from_known_i32`
- `cheatscan_new_from_known_f32`

### Scanning

Previous-value scan:

```c
uint8_t cheatscan_scan_previous(
  Scanner *scanner,
  const uint8_t *next_block_ptr,
  size_t next_block_len,
  uint8_t cmp);
```

Exact-value scans are typed:

- `cheatscan_scan_u8`
- `cheatscan_scan_u16`
- `cheatscan_scan_u32`
- `cheatscan_scan_i8`
- `cheatscan_scan_i16`
- `cheatscan_scan_i32`
- `cheatscan_scan_f32`

Each scan function returns `0` on success, or a `ScanError` code on failure.

### Reading Results

Candidate count:

```c
uint32_t cheatscan_count(Scanner *scanner);
```

Copy result addresses into a caller-provided buffer:

```c
size_t cheatscan_write_results(
  Scanner *scanner,
  uint32_t *out_results_ptr,
  size_t out_results_len,
  size_t offset);
```

This lets you page through results without exposing raw internal pointers.

### Lifetime

Destroy a scanner with:

```c
void cheatscan_free(Scanner *scanner);
```

## Build

Build the Rust crate:

```bash
cargo build
```

Run tests:

```bash
cargo test
```

Build release artifacts:

```bash
cargo build --release
```

The crate is currently configured to produce Rust library artifacts, and the repository also contains generated C headers in [`include/`](/Users/red/Projects/cheatscan/include).

## Current Status

`cheatscan` is currently being shaped as the core scanning library first.

The Rust API and the C-facing header are the primary surfaces for now. Higher-level bindings can be added later once the core API is considered stable.
