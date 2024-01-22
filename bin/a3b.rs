// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal


// cargo test --bin a3b
#[derive(Debug, PartialEq)]
enum ComparisonTo5 {
    LowerThan5,
    HigherThan5,
    EqualTo5,
}

fn sample_fn(n: u8) -> ComparisonTo5 {
    use ComparisonTo5::*;
    match n {
        0..=4 => LowerThan5,
        5 => EqualTo5,
        6.. => HigherThan5
    }
}

fn main() {
    // use `cargo test --bin m5-tests` to check your work
}

#[cfg(test)]
mod test {
    use super::{sample_fn, ComparisonTo5};

    macro_rules! multi_test {
        (
            $fn:ident:
            $( $name:ident -> ($input:expr, $expected:expr) ),+
            $(,)? //optional comma
        ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($fn($input), $expected);
                }
            )+
        }
    }

    multi_test!(sample_fn:
        sample_fn_lower_0 -> (0, ComparisonTo5::LowerThan5),
        sample_fn_lower_4 -> (4, ComparisonTo5::LowerThan5),
        sample_fn_equal -> (5, ComparisonTo5::EqualTo5),
        sample_fn_higher_6 -> (6, ComparisonTo5::HigherThan5),
        sample_fn_higher_199 -> (199, ComparisonTo5::HigherThan5),
    );
}