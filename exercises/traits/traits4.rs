// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a hint.


pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// Instead of supplying a Type for the arguments 'software' and 'software_two',
// we can require that the argument implements a given Trait, independent of the Type.
// using the impl keyword followed by the Trait name in place of the Type
// This allows more flexibility with argument Types, avoiding issues where the arguments
// are swapped like in the test below
fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
