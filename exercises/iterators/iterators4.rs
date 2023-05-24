// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // the .fold() methods signature requires 2 arguments, an initial value
    // and a closure that also has 2 arguments.
    // the 2 arguments for the closure are the accumulator and a variable that
    // represents each element in the Iterator
    // Knowing this, calculating for factorial is as easy as multiplying each
    // element with the accumulated results
    // the range (1..=num) included the = sign to specify that num is included
    // in the range
    (1..=num).fold(1, |acc,x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
