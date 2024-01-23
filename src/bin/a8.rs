// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


// cargo run --bin a8

enum flavors {
    Orange,
    Grape,
    Apple,
    Cherry,
    Strawberry,
}

struct drink {
    flavor: flavors,
    ounces: f64,
}

fn print_flavor_weight(drink: drink) {
    match drink.flavor {
        flavors::Orange => println!("Orange: {} ounces", drink.ounces),
        flavors::Grape => println!("Grape: {} ounces", drink.ounces),
        flavors::Apple => println!("Apple: {} ounces", drink.ounces),
        flavors::Cherry => println!("Cherry: {} ounces", drink.ounces),
        flavors::Strawberry => println!("Strawberry: {} ounces", drink.ounces),
    }
}

fn main() {

    let orange = drink {
        flavor: flavors::Orange,
        ounces: 12.0,
    };

    let grape = drink {
        flavor: flavors::Grape,
        ounces: 12.0,
    };

    let apple = drink {
        flavor: flavors::Apple,
        ounces: 12.0,
    };

    let cherry = drink {
        flavor: flavors::Cherry,
        ounces: 12.0,
    };

    let strawberry = drink {
        flavor: flavors::Strawberry,
        ounces: 12.0,
    };

    print_flavor_weight(orange);
    print_flavor_weight(grape);
    print_flavor_weight(apple);
    print_flavor_weight(cherry);
    print_flavor_weight(strawberry);

}
