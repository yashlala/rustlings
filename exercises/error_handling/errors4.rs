// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

use std::cmp::Ordering;
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match 0.cmp(&value) {
            Ordering::Less => Ok(PositiveNonzeroInteger(value as u64)),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Greater => Err(CreationError::Negative),
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
