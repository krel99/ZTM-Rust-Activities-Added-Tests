// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    let value = 150;
    let is_big = if value > 100 { true } else { false };
    print_message(is_big);
}

fn print_message(is_big: bool) {
    match is_big {
        true => println!("its big"),
        false => println!("its small"),
    }
}

