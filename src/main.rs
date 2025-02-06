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
// fn main() {
//     // Define the struct first, outside the main function
//     struct User {
//         name: String, //stores in heap ; the len; pointer and capacity will be stored in stack and which points to stack
//         ph_no: u64,   // stores in stack
//                       // check notes in obsidian
//     }

//     // Create an instance of User
//     let name: String = String::from("Ashwin");
//     let user = User { name, ph_no: 12345 };

//     // Access the struct fields using dot notation
//     println!(
//         "The name is {} and his phone-number is {}",
//         user.name, user.ph_no
//     );
// }

// Implement in struct

// fn main() {
//     struct Add {
//         var_a: u64,
//         var_b: u64,
//     }

//     impl Add {
//         fn add_impl(&self) -> u64 {
//             &self.var_a + &self.var_b
//         }
//     }

//     let add_operation = Add {
//         var_a: 32,
//         var_b: 10,
//     };

//     println!(
//         "This is the answer for add_operation implemented using impl in structs {}",
//         add_operation.add_impl()
//     );
// }

// Enum in Rust
// fn main() {
//     enum Colors {
//         Red,
//         Yellow,
//         Blue,
//     }

//     // Fixed parameter syntax and renamed to 'color'
//     fn choose_color(color: Colors) {
//         // Using match for enum comparison
//         match color {
//             Colors::Red => println!("user chose red"),
//             Colors::Yellow => println!("user chose yellow"),
//             Colors::Blue => println!("user chose blue"),
//         }
//     }

//     // Example usage
//     let selected_color = Colors::Red;
//     choose_color(selected_color);
//     // println!("{}", Colors::Red);
// }

// Pattern matching in Rust

// fn main() {
//     enum Shapes {
//         Square(f64),
//         Rectangle(f64, f64),
//     };

//     fn calculate_area_ofTheShapes(shape: Shapes) -> f64 {
//         match shape {
//             Shapes::Square(side) => side * side,
//             Shapes::Rectangle(width, height) => width * height,
//         }
//     }

//     let Sqaure_input = calculate_area_ofTheShapes(Shapes::Square((40.2)));
//     let Rectangle_input = calculate_area_ofTheShapes(Shapes::Rectangle((30.5), (40.5)));

//     println!("{} This is the output of the square_Area", Sqaure_input);

//     println!(
//         "{} This is the output of the rectangle_Area",
//         Rectangle_input
//     );
// }

//Result enum (error handling concepts in rust)
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// } // Result is an enum provided by rust with generic types
//these generic types if T being the string then another variable having T should also be string
// generic types can be any types

// // generic types
// fn main() {
//     //Correct usage of generic types in simplestruct1
//     //if we define A being string and B being some int (throws up error)
//     //This below struct having a single generic Type being A (and assigning it to varible means both should be in the same type)
//     struct SimpleStruct1<A> {
//         var_a: A,
//         var_b: A,
//     }

//     let struct_defining_1 = SimpleStruct1 {
//         var_a: "ashwin",
//         var_b: "dev",
//     };
//     println!("{}", struct_defining_1.var_a);
//     println!("{}", struct_defining_1.var_b);

//     struct SimpleStruct2<A> {
//         var_c: A,
//         var_d: A,
//     }
//     let struct_defining_2 = SimpleStruct2 {
//         var_c: 1,
//         var_d: "ashwin", // This SimpleStruct2 will work because struct_defining2 has two generic types defined in thats A being one type and B beign another
//     };

//     println!(
//         "{} incorrect usage of generic types",
//         struct_defining_2.var_d
//     );

// uncomment the struct_defining_2 which is incorrect usage of simpleStruct2
// }

// Result enum in rust (error handling)
fn main() {}
