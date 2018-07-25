#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    variable_scope();
    string_from_scope();
    ways_vars_and_data_interact_move();
    ways_vars_and_data_interact_clone();
    stack_only_data_copy();
    ownership_and_functions();
    return_values_and_scope();
    return_multiple_values_with_tuples();
}

fn variable_scope() {
    { //s is not valid here, it's not yet declared
        let s = "Hello"; // s is valid from this point forward

        // do stuff with s
    } // This scope is now over, and s is no longer valid
}

fn string_from_scope() {
    let s = String::from("Hello"); // s is valid from this point forward

    // do stuff with s
} // This scope is now over, and s is no longer valid


fn ways_vars_and_data_interact_move() {
    basic();
}

fn basic() {
    // This two integers are simple data structures so
    // the get pushed onto the stack and y is a copy
    // of x (namely the value 5)
    let x = 5;
    let y = x;
}


fn string() {
    // A string is made up of 3 parts: pointer, length, capacity
    //
    // The pointer to the memory that holds the contents of the string
    // a length of bytes that the string is currently using
    // capacity is the total amount of memory in bytes that the string is using
    
    let s1 = String::from("hello"); // s1's length and capacity is stored on the stack, however
                                   // its pointer is heap allocated
    let s2 = s1; // When s2 is assigned as s1 the pointer, length, and capacity all get copied to s2.
                 // However, s1 is marked as invalid, so when it goes out of scope rust does not
                 // need to free the allocated memory that s1 is pointing. When you go to use s1
                 // after this assignment rust will give you an error because the compiler will not
                 // allow you to use invalid references like s1. Uncomment code below to see
    //println!("{}", s1);
}


fn ways_vars_and_data_interact_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // This does copy the data from the heap
                        // when you call clone on some arbitrary code
                        // that is being executed, that code might
                        // be expensive. So we know something different
                        // is going on
                        // Uncomment code to see it run
    
    //println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_only_data_copy() {
    let x = 5;
    let y = x; // We don't have to call clone here in order to have x still be valid
              // after moving y. That is because types like integers have a known size
              // at compile time and are stored on the stack, thus making copies are
              // very quick. This goes into a more advanced Rust feature called traits.
              // One of the traits is called `Copy`, which basically means the type has
              // the ability to be valid after move. If we try to add a `Copy` trait to a type
              // implements the `Drop`, a type who has members that implement the `Drop` trait
              // we will get a compile time error. Types that do not do memory allocation
              // can have the `Copy` trait. Some data types that have this trait are:
              // All integers
              // Boolean types
              // Char types
              // Tuples, but only if they contain types that are also `Copy`
              // Uncomment code to see it run!

    //println!("x = {}, y = {}", x, y);
}


fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value move into the function
                       // and is no longer valid in this scope
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function
                  // but i32 is Copy, so it's okay to still be
                  // valid and used after this call
} // x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    //println!("{}", some_string);
} // here some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    //println!("{}", some_integer);
} // Here, some_integer goes out of scope, nothing special happens

fn return_values_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return value to s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back,
                                      // which also moves it return value into s3
} // Here s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens
// s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its return value into the caller
    let some_string = String::from("Hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moved out into the calling scope
}

fn return_multiple_values_with_tuples() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    // println!("Then length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

