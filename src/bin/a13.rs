// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

enum NumberStringOrInt {
    Integer(i32),
    String(String),
}

fn print_number(array_of_numbers: Vec<NumberStringOrInt>) {
    for number in &array_of_numbers {
        match number {
            NumberStringOrInt::Integer(n) => println!("{}", n),
            NumberStringOrInt::String(s) => println!("{}", s),
        }
    }
    println!(
        "The total number of elements in the vector is: {}",
        array_of_numbers.len()
    );
}

fn main() {
    let my_vector: Vec<NumberStringOrInt> = vec![
        NumberStringOrInt::Integer(1),
        NumberStringOrInt::Integer(2),
        NumberStringOrInt::String("Thirty".to_string()),
        NumberStringOrInt::Integer(4),
    ];

    print_number(my_vector);
}

#[test]
fn run_with_defaults() {}

//cargo test --bin a13
