// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// cargo test --bin a12
// cargo run --bin a12

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn print(&self){
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
        }
    }
}

struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimensions {
    fn print(&self){
        println!("Length: {}", self.length);
        println!("Width: {}", self.width);
        println!("Height: {}", self.height);
    }
}
struct ShippingBox {
    weight: u32,
    color: Color,
    dimensions: Dimensions,
}
impl ShippingBox {
    fn new(weight: u32, color: Color, dimensions: Dimensions) -> ShippingBox {
        ShippingBox {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self){
        println!("Weight: {}", self.weight);
        self.color.print();
        self.dimensions.print();
    }
}

fn main() {
    let box1 = ShippingBox::new(10, Color::Red, Dimensions{length: 10, width: 10, height: 10});
    box1.print();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_shipping_box_new() {
//         // Create dimensions
//         let dimensions = Dimensions {
//             length: 20,
//             width: 30,
//             height: 40,
//         };

//         // Create a new ShippingBox
//         let new_box = ShippingBox::new(15, Color::Red, dimensions);

//         // Assert correct values
//         assert_eq!(new_box.weight, 15);
//         match new_box.color {
//             Color::Red => (), // Correct color
//             // _ would mean any other color which is not expected
//             _ => panic!("Color is not Red"),
//         }
        
//         assert_eq!(new_box.dimensions.length, 20);
//         assert_eq!(new_box.dimensions.width, 30);
//         assert_eq!(new_box.dimensions.height, 40);
//     }
// }

#[cfg(test)]
mod tests {
    // use super::*;
    use super::{Color, Dimensions, ShippingBox};
    macro_rules! test_shipping_box_new {
        (
            $fn:ident:
            $( $name:ident -> ($input1:expr, $input2:expr, Dimensions{length: $input3:expr, width: $input4:expr, height: $input5:expr})),+
            $(,)?
        ) => {
            $(
                #[test]
                fn $name() {
                    let expected_weight = $input1;
                    let expected_color = $input2;
                    let expected_length = $input3;
                    let expected_width = $input4;
                    let expected_height = $input5;
                    let new_box = ShippingBox::new($input1, $input2, Dimensions{length: $input3, width: $input4, height: $input5});
                    assert_eq!(new_box.weight, expected_weight);
                    assert!(matches!(new_box.color, expected_color));
                    assert_eq!(new_box.dimensions.length, expected_length);
                    assert_eq!(new_box.dimensions.width, expected_width);
                    assert_eq!(new_box.dimensions.height, expected_height);
                }
            )+
        }
    }
        test_shipping_box_new!(shipping_box_test:
            a1 -> (10, Color::Red, Dimensions{length: 10, width: 10, height: 10}),
            a2 -> (10, Color::Green, Dimensions{length: 10, width: 100, height: 10}),
            a3 -> (15, Color::Blue, Dimensions{length: 100, width: 10, height: 10}),
            a4 -> (20, Color::Red, Dimensions{length: 10, width: 10, height: 1000}),
        );
    

}
//cargo test --bin a12