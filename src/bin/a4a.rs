// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let condition = true; // You can change this to false to test the other branch

    let message = match condition {
        true => "it's true",
        false => "it's false",
    };

    println!("{}", message);
}

