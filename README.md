# ensure-cov

`ensure-cov` is a simple Rust crate designed to help ensure test coverage in
your projects. It provides a set of functions to track and assert code coverage
during test execution.

## Features

- Track code coverage for specific named sections
- Assert that certain code paths have been covered
- Check the number of times a specific section has been covered
- Clear coverage data

## Usage

The crate's functionality is enabled when the crate is compiled in test or debug
mode. In release builds or when the `disable` feature is enabled, all functions
become no-ops, ensuring zero overhead in production.

## API

When enabled, the crate provides the following functions:

- `notify_cov(name: &str)`: Increment the coverage count for a named section.
- `assert_cov(name: &str)`: Assert that a named section has been covered at
  least once.
- `assert_cov_at_least(name: &str, expected: usize)`: Assert that a named
  section has been covered at least `expected` number of times.
- `get_cov_for(name: &str) -> usize`: Get the current coverage count for a named
  section.
- `clear_cov()`: Clear all coverage data.

## Example

```rust
use ensure_cov::*;

#[test]
fn test_coverage() {
    notify_cov("section_a");
    notify_cov("section_b");
    notify_cov("section_b");

    assert_cov("section_a");
    assert_cov_at_least("section_b", 2);

    assert_eq!(get_cov_for("section_a"), 1);
    assert_eq!(get_cov_for("section_b"), 2);

    clear_cov();
    assert_eq!(get_cov_for("section_a"), 0);
}
```
