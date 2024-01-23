// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

// cargo run --bin a11

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn print_quantity(item: &GroceryItem) {
    println!("{}", item.quantity );
}

fn print_id(item:&GroceryItem) {
    println!("{}", item.id)
}

fn main() {
    let item = GroceryItem {
        quantity: 5,
        id: 1234,
    };
    print_quantity(&item);
    print_id(&item);
}

