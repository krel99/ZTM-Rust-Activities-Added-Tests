// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

// cargo test --bin a4b

fn make_a_decision(number: i32) -> String {
    let string = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "not 1-3"
    };
    string.to_string()
}

fn main() {
    println!("{}", make_a_decision(1));
    println!("{}", make_a_decision(2));
    println!("{}", make_a_decision(3));
}


#[cfg(test)]
mod test {
    use super::make_a_decision;

    macro_rules! multi_test {
        (
            $fn:ident:
            $( $name:ident -> ($input:expr, $expected:expr) ),+
            $(,)? //optional comma
        ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($fn($input).as_str(), $expected);
                }
            )+
        }
    }

    multi_test!(make_a_decision:
        make_a_decision_1 -> (1, "one"),
        make_a_decision_2 -> (2, "two"),
        make_a_decision_3 -> (3, "three"),
        make_a_decision_6 -> (6, "not 1-3"),
    );
}