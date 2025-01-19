// // fn main() {
// //     println!("Hello, world!");
// // }

// fn main() {
//     let x: i32 = 54;
//     println!("This is the value of x which is {x}"); // its more like using placeholder, in C to separate the
//                                                      // variable from the string, we use %d, %s, %f, etc. but in Rust, we use {variable_name}
//                                                      // we are defining the type as i32 which is nothing but signed integer [32 bit in memory is needed in the ram of the machine]
//                                                      // int32 defines the signed integer
//                                                      // u32 defines the unsigned integer
//                                                      // f32 defines the floating point number
// }

// fn main() {
//     // let is_male = false;
//     let is_male = true;
//     let is_above_18 = true;

//     if is_male {
//         println!("You are a male");
//     } else {
//         println!("You are not a male");
//     }

//     if is_male && is_above_18 {
//         print!("You are a legal male adult");
//     }
// }

// Strings in Rust

fn main() {
    // let greeting = "Hello, world!"; // its stack allocated

    let greeting = String::from("hello world"); // its heap allocated (growing and dynamic allocation)
    println!("{}", greeting);

    //printing the first character of the string greeting
    // print!("{}", greeting.chars().nth(1)); // this will throw up an error since rust is a strongly typed language ; and tells in a way that it is not sure of the type of the variable

    let char1 = greeting.chars().nth(1).unwrap(); // this will print the second character of the string greeting
    print!("{}", char1); // it prints the second character of the string greeting ; since we have used unwrap() method, it will not throw up an error ;why? because unwrap() method is used to handle the error

    //  What unwrap() Does
    // The unwrap() method is called on an Option (or Result) to extract the value if it exists. It has the following behavior:

    // If the value is Some(value), unwrap() returns the value.
    // If the value is None, unwrap() panics (causes the program to crash) with an error message that you provide.
}
