// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

const FIRST_NAME: &str = "Karel";
const LAST_NAME: &str = "S";

fn print_first_name() {
    println!("My first name is {}", FIRST_NAME);
}

fn print_last_name() {
    println!("Last name is {}", LAST_NAME);
}

fn main(){
    print_first_name();
    print_last_name();
}
