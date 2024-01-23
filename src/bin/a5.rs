// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

// cargo test --bin a5
// cargo run --bin a5

fn loop_from_to(from: i8, to: i8) -> Vec<i8> {
    let mut counter = from;
    let mut my_vector: Vec<i8> = Vec::new();
    while counter < to {
        my_vector.push(counter);
        println!("{}", counter.to_string());
        counter += 1;
    }
    my_vector
}
fn main() {
    let my_returned_vector = loop_from_to(0, 10);
}

#[cfg(test)]
mod tests {
    use super::loop_from_to;

    macro_rules! multi_test_length {
        (
            $fn:ident:
            $( $name:ident -> ($input1:expr, $input2:expr)),+
            $(,)? //optional comma
        ) => {
            $(
                #[test]
                fn $name() {
                    let expected_length = if $input1 < $input2 { $input2 - $input1 } else { 0 };
                    assert_eq!($fn($input1, $input2).len(), expected_length as usize);
                }
            )+
        }
    }

    multi_test_length!(loop_from_to:
        loop_from_to_1 -> (1, 4), // Tests for vector with elements 1, 2, 3
        loop_from_to_2 -> (2, 5), // Tests for vector with elements 2, 3, 4
    );
}