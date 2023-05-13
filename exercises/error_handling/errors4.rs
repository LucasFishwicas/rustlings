// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError { // Created an enum to represent a kind of error
    Negative,
    Zero,
}
// Define method new(v) for the PositiveNonzeroInteger struct - using the impl keyword
impl PositiveNonzeroInteger {
    // Ok() contains the created struct, Err() contains the created enum
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // If the value is positive, return the created struct holding the value
        // Else if negative or zero, return created enum with relevant variants
        if value > 0  {
            Ok(PositiveNonzeroInteger(value as u64))
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Err(CreationError::Zero)
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
