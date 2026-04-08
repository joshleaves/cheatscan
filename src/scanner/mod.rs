pub mod alignment;
pub mod comparison;
pub mod configuration;
pub mod endianness;
pub mod scan_error;
pub mod scan_value;
pub mod value_reader;
pub mod value_type;

pub use alignment::Alignment;
pub use comparison::ComparisonType;
pub use configuration::Configuration;
pub use endianness::Endianness;
pub use scan_error::ScanError;
pub use scan_value::ScanValue;
pub use value_type::ValueType;

use crate::scanner::value_reader::{
  read_f32, read_i8, read_i16, read_i32, read_u8, read_u16, read_u32,
};

/// Stateful memory scanner that keeps the previous RAM block and the current candidate set.
pub struct Scanner {
  value_type: ValueType,
  endianness: Endianness,
  alignment: Alignment,
  base_address: u32,
  width: usize,
  results: Vec<u32>,
  ram_block: Vec<u8>,
  has_filtered: bool,
}

impl Scanner {
  /// Builds a valid scanner state from an initial RAM block.
  ///
  /// This is the common constructor used by the public initialization paths.
  fn new(config: Configuration, initial_block: &[u8]) -> Result<Self, ScanError> {
    let width = config.value_type.width();
    if initial_block.len() < width {
      return Err(ScanError::RamBlockTooSmall);
    }

    Ok(Self {
      value_type: config.value_type,
      endianness: config.endianness,
      alignment: config.alignment,
      base_address: config.base_address,
      width,
      results: Vec::new(),
      ram_block: initial_block.to_vec(),
      has_filtered: false,
    })
  }

  /// Creates a scanner seeded with an initial RAM block, without applying any filter yet.
  pub fn new_from_unknown(config: Configuration, initial_block: &[u8]) -> Result<Self, ScanError> {
    Self::new(config, initial_block)
  }

  /// Creates a scanner and immediately applies a first exact-value scan on the provided block.
  pub fn new_from_known(
    config: Configuration,
    initial_block: &[u8],
    cmp: ComparisonType,
    value: ScanValue,
  ) -> Result<Self, ScanError> {
    if matches!(value, ScanValue::PreviousValue) {
      return Err(ScanError::InitialScanValueRequired);
    }

    let mut scanner = Self::new(config, initial_block)?;
    scanner.scan(initial_block, cmp, value)?;
    Ok(scanner)
  }

  /// Applies a new scan against `next_block`.
  ///
  /// `next_block` must have the same length as the block used to initialize the scanner.
  pub fn scan(
    &mut self,
    next_block: &[u8],
    cmp: ComparisonType,
    value: ScanValue,
  ) -> Result<(), ScanError> {
    self.ensure_ram_block_len_matches(next_block)?;
    self.ensure_scan_value_matches_config(&value)?;

    match (self.value_type, value) {
      (ValueType::U8, ScanValue::U8(expected)) => {
        self.filter_results(Some(next_block), read_u8, cmp, Some(expected));
      }
      (ValueType::U8, ScanValue::PreviousValue) => {
        self.filter_results(Some(next_block), read_u8, cmp, None);
      }
      (ValueType::U16, ScanValue::U16(expected)) => {
        self.filter_results(Some(next_block), read_u16, cmp, Some(expected));
      }
      (ValueType::U16, ScanValue::PreviousValue) => {
        self.filter_results(Some(next_block), read_u16, cmp, None);
      }
      (ValueType::U32, ScanValue::U32(expected)) => {
        self.filter_results(Some(next_block), read_u32, cmp, Some(expected));
      }
      (ValueType::U32, ScanValue::PreviousValue) => {
        self.filter_results(Some(next_block), read_u32, cmp, None);
      }
      (ValueType::I8, ScanValue::I8(expected)) => {
        self.filter_results(Some(next_block), read_i8, cmp, Some(expected));
      }
      (ValueType::I8, ScanValue::PreviousValue) => {
        self.filter_results(Some(next_block), read_i8, cmp, None);
      }
      (ValueType::I16, ScanValue::I16(expected)) => {
        self.filter_results(Some(next_block), read_i16, cmp, Some(expected));
      }
      (ValueType::I16, ScanValue::PreviousValue) => {
        self.filter_results(Some(next_block), read_i16, cmp, None);
      }
      (ValueType::I32, ScanValue::I32(expected)) => {
        self.filter_results(Some(next_block), read_i32, cmp, Some(expected));
      }
      (ValueType::I32, ScanValue::PreviousValue) => {
        self.filter_results(Some(next_block), read_i32, cmp, None);
      }
      (ValueType::F32, ScanValue::F32(expected)) => {
        self.filter_results(Some(next_block), read_f32, cmp, Some(expected));
      }
      (ValueType::F32, ScanValue::PreviousValue) => {
        self.filter_results(Some(next_block), read_f32, cmp, None);
      }
      _ => unreachable!("scan value compatibility is validated before dispatch"),
    }

    self.ram_block.copy_from_slice(next_block);

    Ok(())
  }

  /// Applies a new scan against the current block.
  pub fn scan_again(&mut self, cmp: ComparisonType, value: ScanValue) -> Result<(), ScanError> {
    self.ensure_scan_value_matches_config(&value)?;
    if matches!(value, ScanValue::PreviousValue) {
      return Err(ScanError::PreviousValueRequiresNewBlock);
    }

    match value {
      ScanValue::U8(expected) => {
        self.filter_results(None, read_u8, cmp, Some(expected));
      }
      ScanValue::U16(expected) => {
        self.filter_results(None, read_u16, cmp, Some(expected));
      }
      ScanValue::U32(expected) => {
        self.filter_results(None, read_u32, cmp, Some(expected));
      }
      ScanValue::I8(expected) => {
        self.filter_results(None, read_i8, cmp, Some(expected));
      }
      ScanValue::I16(expected) => {
        self.filter_results(None, read_i16, cmp, Some(expected));
      }
      ScanValue::I32(expected) => {
        self.filter_results(None, read_i32, cmp, Some(expected));
      }
      ScanValue::F32(expected) => {
        self.filter_results(None, read_f32, cmp, Some(expected));
      }
      _ => unreachable!("scan value compatibility is validated before dispatch"),
    }

    Ok(())
  }

  /// Returns the number of current candidates.
  ///
  /// Before the first filtering pass, this returns the implicit candidate count derived from the
  /// configured alignment and value width.
  pub fn count(&self) -> usize {
    if self.has_filtered {
      self.results.len()
    } else {
      self.candidate_count()
    }
  }

  /// Returns an iterator over materialized result addresses.
  ///
  /// The returned addresses already include `base_address`.
  pub fn results(&self) -> impl Iterator<Item = u32> + '_ {
    self.results.iter().map(|result| result + self.base_address)
  }

  fn ensure_ram_block_len_matches(&self, next_block: &[u8]) -> Result<(), ScanError> {
    if next_block.len() != self.ram_block.len() {
      return Err(ScanError::InvalidRamBlockLength);
    }
    Ok(())
  }

  /// Ensures that an exact scan value matches the configured value type.
  ///
  /// Previous-value scans are always accepted because the comparison value is read from the
  /// scanner's stored RAM block using the configured type.
  fn ensure_scan_value_matches_config(&self, value: &ScanValue) -> Result<(), ScanError> {
    match value.value_type() {
      None => Ok(()), // PreviousValue
      Some(t) if t == self.value_type => Ok(()),
      _ => Err(ScanError::TypeMismatch),
    }
  }

  /// Filters candidates by comparing the current value against either a constant value or the previous block.
  fn filter_results<T, R>(
    &mut self,
    next_block: Option<&[u8]>,
    read: R,
    cmp: ComparisonType,
    expected: Option<T>,
  ) where
    R: Fn(&[u8], Endianness) -> T,
    T: PartialOrd + PartialEq + Copy,
  {
    let endianness = self.endianness;
    let previous_block = self.ram_block.as_slice();
    let cmp_fn = cmp.to_fn();

    let matches = |offset: usize| {
      let end = offset + self.width;
      let candidate = match next_block {
        Some(next) => read(&next[offset..end], endianness),
        None => read(&self.ram_block[offset..end], endianness),
      };
      let rhs = match expected {
        Some(expected) => expected,
        None => read(&previous_block[offset..end], endianness),
      };
      cmp_fn(candidate, rhs)
    };

    if self.has_filtered {
      self.results.retain(|offset| matches(*offset as usize));
      return;
    }

    let mut results = Vec::new();
    self.candidates_filter(|offset| {
      if matches(offset) {
        results.push(offset as u32);
      }
    });
    self.results = results;
    self.has_filtered = true;
  }

  /// Iterates over the active candidate offsets.
  ///
  /// Before any filtering has happened, this walks the implicit full candidate space.
  fn candidates_filter(&self, mut visit: impl FnMut(usize)) {
    if self.has_filtered {
      for &offset in &self.results {
        visit(offset as usize);
      }
      return;
    }

    let step = match self.alignment {
      Alignment::Aligned => self.width,
      Alignment::Unaligned => 1,
    };

    for offset in (0..=(self.ram_block.len() - self.width)).step_by(step) {
      visit(offset);
    }
  }

  /// Computes the implicit candidate count for an unfiltered scanner state.
  fn candidate_count(&self) -> usize {
    let step = match self.alignment {
      Alignment::Aligned => self.width,
      Alignment::Unaligned => 1,
    };

    ((self.ram_block.len() - self.width) / step) + 1
  }
}

#[cfg(test)]
mod tests {
  use super::Scanner;
  use crate::scanner::{
    Alignment, ComparisonType, Configuration, Endianness, ScanError, ScanValue, ValueType,
  };

  fn config(value_type: ValueType) -> Configuration {
    Configuration {
      value_type,
      endianness: Endianness::Little,
      alignment: Alignment::Aligned,
      base_address: 0x8000,
    }
  }

  #[test]
  fn new_from_unknown_rejects_too_small_ram_block() {
    let scanner = Scanner::new_from_unknown(config(ValueType::U16), &[0x12]);

    assert!(matches!(scanner, Err(ScanError::RamBlockTooSmall)));
  }

  #[test]
  fn new_from_unknown_reports_implicit_candidate_count_before_first_filter() {
    let scanner = Scanner::new_from_unknown(config(ValueType::U16), &[0, 1, 2, 3, 4, 5]).unwrap();

    assert_eq!(scanner.count(), 3);
    assert_eq!(scanner.results().count(), 0);
  }

  #[test]
  fn new_from_known_materializes_matching_results() {
    let scanner = Scanner::new_from_known(
      config(ValueType::U16),
      &[1, 0, 2, 0, 1, 0],
      ComparisonType::Eq,
      ScanValue::U16(1),
    )
    .unwrap();

    let results: Vec<u32> = scanner.results().collect();

    assert_eq!(scanner.count(), 2);
    assert_eq!(results, vec![0x8000, 0x8004]);
  }

  #[test]
  fn new_from_known_rejects_previous_value() {
    let scanner = Scanner::new_from_known(
      config(ValueType::U16),
      &[1, 0, 2, 0],
      ComparisonType::Eq,
      ScanValue::PreviousValue,
    );

    assert!(matches!(scanner, Err(ScanError::InitialScanValueRequired)));
  }

  #[test]
  fn scan_previous_value_filters_against_stored_ram_block() {
    let mut scanner = Scanner::new_from_unknown(config(ValueType::U8), &[10, 20, 30, 40]).unwrap();

    scanner
      .scan(
        &[10, 19, 31, 40],
        ComparisonType::Gt,
        ScanValue::PreviousValue,
      )
      .unwrap();

    let results: Vec<u32> = scanner.results().collect();

    assert_eq!(scanner.count(), 1);
    assert_eq!(results, vec![0x8002]);
  }

  #[test]
  fn scan_exact_after_previous_value_reuses_materialized_results() {
    let mut scanner = Scanner::new_from_unknown(config(ValueType::U8), &[10, 20, 30, 40]).unwrap();

    scanner
      .scan(
        &[10, 19, 31, 40],
        ComparisonType::Gt,
        ScanValue::PreviousValue,
      )
      .unwrap();
    scanner
      .scan(&[10, 19, 31, 40], ComparisonType::Eq, ScanValue::U8(31))
      .unwrap();

    let results: Vec<u32> = scanner.results().collect();

    assert_eq!(scanner.count(), 1);
    assert_eq!(results, vec![0x8002]);
  }

  #[test]
  fn scan_again_refines_results_on_current_block() {
    let mut scanner = Scanner::new_from_unknown(config(ValueType::U8), &[10, 20, 30, 40]).unwrap();

    scanner
      .scan(
        &[10, 19, 31, 40],
        ComparisonType::Gt,
        ScanValue::PreviousValue,
      )
      .unwrap();
    scanner
      .scan_again(ComparisonType::Eq, ScanValue::U8(31))
      .unwrap();

    let results: Vec<u32> = scanner.results().collect();

    assert_eq!(scanner.count(), 1);
    assert_eq!(results, vec![0x8002]);
  }

  #[test]
  fn scan_again_can_chain_on_current_block() {
    let mut scanner = Scanner::new_from_unknown(config(ValueType::U8), &[10, 20, 30, 40]).unwrap();

    scanner
      .scan(
        &[10, 19, 31, 40],
        ComparisonType::Gt,
        ScanValue::PreviousValue,
      )
      .unwrap();
    scanner
      .scan_again(ComparisonType::Gt, ScanValue::U8(20))
      .unwrap();
    scanner
      .scan_again(ComparisonType::Lt, ScanValue::U8(32))
      .unwrap();

    let results: Vec<u32> = scanner.results().collect();

    assert_eq!(scanner.count(), 1);
    assert_eq!(results, vec![0x8002]);
  }

  #[test]
  fn scan_again_rejects_previous_value() {
    let mut scanner = Scanner::new_from_unknown(config(ValueType::U8), &[10, 20, 30, 40]).unwrap();

    scanner
      .scan(
        &[10, 19, 31, 40],
        ComparisonType::Gt,
        ScanValue::PreviousValue,
      )
      .unwrap();

    let error = scanner.scan_again(ComparisonType::Eq, ScanValue::PreviousValue);

    assert_eq!(error, Err(ScanError::PreviousValueRequiresNewBlock));
  }

  #[test]
  fn scan_again_rejects_mismatched_value_type() {
    let mut scanner = Scanner::new_from_unknown(config(ValueType::U16), &[1, 0, 2, 0]).unwrap();

    let error = scanner.scan_again(ComparisonType::Eq, ScanValue::U8(1));

    assert_eq!(error, Err(ScanError::TypeMismatch));
  }

  #[test]
  fn scan_rejects_mismatched_value_type() {
    let mut scanner = Scanner::new_from_unknown(config(ValueType::U16), &[1, 0, 2, 0]).unwrap();

    let error = scanner.scan(&[1, 0, 2, 0], ComparisonType::Eq, ScanValue::U8(1));

    assert_eq!(error, Err(ScanError::TypeMismatch));
  }

  #[test]
  fn scan_rejects_wrong_ram_block_length() {
    let mut scanner = Scanner::new_from_unknown(config(ValueType::U8), &[1, 2, 3]).unwrap();

    let error = scanner.scan(&[1, 2], ComparisonType::Eq, ScanValue::U8(1));

    assert_eq!(error, Err(ScanError::InvalidRamBlockLength));
  }
}
