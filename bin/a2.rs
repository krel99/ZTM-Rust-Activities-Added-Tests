// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn addition(f: i32, s: i32) -> i32 {
    f + s
}

fn display_results(r: i32) {
    println!("The result is {:?}", r);
}

fn main() {
    let first = 10;
    let second = 20;
    let result = addition(first, second);
    display_results(result);
}
