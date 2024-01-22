// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

// cargo run --bin a9

fn main() {
    let first_coordinate = get_coordinate(10, 11);
    let (x, y) = first_coordinate;
    if y > 5 {
        println!("The y-value is greater than 5.");
    } else if y < 5 {
        println!("The y-value is less than 5.");
    } else {
        println!("The y-value is equal to 5.");
    }
}

fn get_coordinate(x:i32, y:i32) -> (i32, i32) {
    (x, y)
}