// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// cargo run --bin a7
enum Color {
    Red, Blue, Orange, Green
}

fn print_color(color: Color) {
    let print_text = match color {
        Color::Red => "red",
        Color::Blue => "blue",
        Color::Orange => "orange",
        Color::Green => "green",
    };
    println!("{}", print_text);
}

fn main() {
    let color_1 = Color::Red;
    print_color(color_1)
}
