use cheatscan::{
  Alignment, ComparisonType, Configuration, Endianness, ScanValue, Scanner, ValueType,
};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

const BLOCK_SIZE: usize = 8 * 1024 * 1024;

fn bench_u8(c: &mut Criterion) {
  let mut initial_block = vec![0_u8; BLOCK_SIZE];
  let mut lcg_state: u64 = 0x1234_5678_9ABC_DEF0;
  for byte in &mut initial_block {
    lcg_state = lcg_state.wrapping_mul(6364136223846793005).wrapping_add(1);
    *byte = (lcg_state >> 24) as u8;
  }

  let mut next_block = initial_block.clone();
  for (index, byte) in next_block.iter_mut().enumerate() {
    if index % 7 == 0 {
      *byte = byte.wrapping_add(1);
    } else if index % 17 == 0 {
      *byte = byte.wrapping_sub(1);
    }
  }

  let config = Configuration {
    value_type: ValueType::U8,
    endianness: Endianness::Little,
    alignment: Alignment::Unaligned,
    base_address: 0,
  };

  let mut group = c.benchmark_group("scanner_u8");
  group.warm_up_time(Duration::from_secs(1));
  group.measurement_time(Duration::from_secs(4));
  group.sample_size(60);
  group.throughput(Throughput::Bytes(BLOCK_SIZE as u64));

  group.bench_function("scan_exact_eq_42", |b| {
    b.iter_batched(
      || Scanner::new_from_unknown(config, &initial_block).unwrap(),
      |mut scanner| {
        scanner
          .scan(&next_block, ComparisonType::Eq, ScanValue::U8(42))
          .unwrap();
        black_box(scanner.count());
      },
      BatchSize::SmallInput,
    );
  });

  group.bench_function("scan_previous_gt", |b| {
    b.iter_batched(
      || Scanner::new_from_unknown(config, &initial_block).unwrap(),
      |mut scanner| {
        scanner
          .scan(&next_block, ComparisonType::Gt, ScanValue::PreviousValue)
          .unwrap();
        black_box(scanner.count());
      },
      BatchSize::SmallInput,
    );
  });

  group.bench_function("scan_again_exact_eq_42", |b| {
    b.iter_batched(
      || {
        let mut scanner = Scanner::new_from_unknown(config, &initial_block).unwrap();
        scanner
          .scan(&next_block, ComparisonType::Eq, ScanValue::U8(42))
          .unwrap();
        scanner
      },
      |mut scanner| {
        scanner
          .scan_again(ComparisonType::Eq, ScanValue::U8(42))
          .unwrap();
        black_box(scanner.count());
      },
      BatchSize::SmallInput,
    );
  });

  group.finish();
}

criterion_group!(benches, bench_u8);
criterion_main!(benches);
