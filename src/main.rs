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

// fn main() {
//     // let greeting = "Hello, world!"; // its stack allocated

//     let greeting = String::from("hello world"); // its heap allocated (growing and dynamic allocation)
//     println!("{}", greeting);

//     //printing the first character of the string greeting
//     // print!("{}", greeting.chars().nth(1)); // this will throw up an error since rust is a strongly typed language ; and tells in a way that it is not sure of the type of the variable

//     let char1 = greeting.chars().nth(1).unwrap(); // this will print the second character of the string greeting
//     print!("{}", char1); // it prints the second character of the string greeting ; since we have used unwrap() method, it will not throw up an error ;why? because unwrap() method is used to handle the error

//     //  What unwrap() Does
//     // The unwrap() method is called on an Option (or Result) to extract the value if it exists. It has the following behavior:

//     // If the value is Some(value), unwrap() returns the value.
//     // If the value is None, unwrap() panics (causes the program to crash) with an error message that you provide.
// }

// //Conditionals and modules and for loops usage in Rust
// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }

// mod char_finder {
//     pub fn get_first_char(sentence: String) -> char {
//         // this function is private until we declare the pub keyword
//         let ans_char = sentence.chars().nth(0).unwrap();
//         return ans_char;
//     }
// }

// mod for_looper_numbers {
//     pub fn number_input(n: i32) {
//         let word: String = String::from("Hello");
//         let count = n as usize;
//         for _i in 0..count {
//             println!("{}{}", word, _i)
//         }
//     }
// }
// fn main() {
//     let result_add: i32 = add(2, 3);
//     println!(" The Result is {  }", result_add);

//     if result_add > 5 {
//         println!("The Result is above 5")
//     } else {
//         println!("The result is below 5")
//     }
//     let sentence = String::from("ashwin");
//     let first_char_found = char_finder::get_first_char(sentence);
//     println!("The first character of the word is {} ", first_char_found);

//     for_looper_numbers::number_input(1000);
// }

//Stack and Heap

// fn main() {
//     stack_fn(); // Call the function that uses stack memory
//     heap_fn(); // Call the function that uses heap memory
//     update_string(); // Call the function that changes size of variable at runtime
// }

// fn stack_fn() {
//     // Declare a few integers on the stack
//     let a = 10;
//     let b = 20;
//     let c = a + b;
//     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// }

// fn heap_fn() {
//     // Create a string, which is allocated on the heap
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let combined = format!("{} {}", s1, s2);
//     println!("Heap function: Combined string is '{}'", combined);
// }

// fn update_string() {
//     // Start with a base string on the heap
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);

//     // Append some text to the string
//     for _ in 0..500 {
//         s.push_str(" and some additional text");
//         // println!("After update: {}", s);
//         println!(
//             "Capacity:{} , Length:{} , Pointer:{:p}",
//             s.capacity(),
//             s.len(),
//             s.as_ptr()
//         );
//     }
// }

// Pointer:0x63a96c338b70
// Pointer:0x63a96c338b80
// Pointer change will happen once the capacity of the heap memory is full

// OWNERSHIP IN RUST

//Passing stack Variables inside functions

// fn main() {
//     let x = 1; // crated on stack
//     let y = 3; // created on stack
//     println!("{}", sum(x, y));
//     println!("Hello, world!");
// }

// fn sum(a: i32, b: i32) -> i32 {
//     let c = a + b;
//     return c;
// }

// Passing strings (heap variables) to functions as args

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}", s1); // This line would cause a compile error because ownership has been moved.
// }
//see notes as well for better understanding (Obsidian)

//clone method in Rust
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("{}", s1); // Compiles now
// }

// a function that takes ownership of a string and returns it back

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = takes_ownership(s1);
//     println!("{}", s2);
// }

// fn takes_ownership(some_string: String) -> String {
//     println!("{}", some_string);
//     return some_string; // return the string ownership back to the original main fn
// }

//Borrowing and Reference
//Reference
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = &s1; //Referencing s1 (s2 is just having a pointer to point out to s1)
//     print!("{}", s1);
// }

//Borrowing

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = &s1; //Referencing s1 (s2 is just having a pointer to point out to s1)
//     print_word(s2);
//     print_word1(s2);

//     println!("{}", s1);
// }
// //Immutable Borrow is happening here ; we can't change the value of s2 ; we can only read the value of s2
// fn print_word(word: &String) {
//     println!("{}", word)
// }
// //Multiple Immutable Borrowing can happen in Rust ; but only one mutable borrowing can happen in Rust
// fn print_word1(word: &String) {
//     println!("{}", word)
// }

//Mutable borrowing
//notice the mut keyword in the function argument that is in update_word function and as well in main function being passed as an argument
// fn main() {
//     let mut s1 = String::from("Hello");
//     update_word(&mut s1);
//     println!("{}", s1);
// }

// fn update_word(word: &mut String) {
//     word.push_str(" World");
// }

// // This avoids the problem of having multiple mutable references to the same data, which can cause synchronization issues. Rust’s ownership system ensures that there is only one mutable reference to a piece of data at a time.

//Struct in rust
fn main() {
    // Define the struct first, outside the main function
    struct User {
        name: String, //stores in heap ; the len; pointer and capacity will be stored in stack and which points to stack
        ph_no: u64,   // stores in stack
                      // check notes in obsidian
    }

    // Create an instance of User
    let name: String = String::from("Ashwin");
    let user = User { name, ph_no: 12345 };

    // Access the struct fields using dot notation
    println!(
        "The name is {} and his phone-number is {}",
        user.name, user.ph_no
    );
}
