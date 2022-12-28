// fn main() {
//     // let mut for changing val
//     // let for const
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");

//     // const
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// }

// // variable shadowing
// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// int float 
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("difference {difference}");

//     // multiplication
//     let product = 4 * 30;
//     println!("product {product}");


//     // division
//     let quotient = 56.7 / 32.2;
//     let floored = 2.0 / 3.0; // Results in 0
//     println!("floored {floored}");


//     // remainder
//     let remainder = 43 % 5;
// }

//bool
// fn main() {
//     let t = true;
//     let f: bool = false; // with explicit type annotation
// }

// characters
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// tuple
// fn main() {
//     let tup = (500, 6.4, 1);
//     let (x, y, z) = tup;
//     println!("The value of y is: {y}");
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;
// }

// array
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first} {second}");
}
