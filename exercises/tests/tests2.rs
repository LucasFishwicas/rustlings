// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail!
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a hint.


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // Much like assert!, the assert_eq! panics depending on the values of
        // its arguments and the compiler provides helpful messages to help see
        // how the values differ
        // It takes 2 arguments and expects both to be equal
        assert_eq!(1,1);
    }
}
