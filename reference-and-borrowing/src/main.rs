#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    start_pass_a_reference();
    bad_borrow();
    good_borrow();
}

fn start_pass_a_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass in a reference to `s1`. 
                                     // so this scope still owns s1

    //println!("The length of '{}' is {}", s1, len);
} // s1 goes out of scope, and since this scope owns s1 it will be dropped.


fn calculate_length(s: &String) -> usize { // s is a reference to a String, that is to say
                                           // this scope is borrowing s1.
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn bad_borrow() {
    let s = String::from("hello"); // s comes into scope
    bad_change(&s); // pass a reference of s to bad_change (compile won't allow this)
}

fn bad_change(s: &String) { // s is a reference to a String.
    //s.push_str(" world"); // we try to mutate the String, but references
                          // are mutable by default. We have to pass a mutable
                          // reference. See good_borrow for example.
}

fn good_borrow() {
    let mut s = String::from("Hello"); // s comes into scope, and its mutable
    good_change(&mut s); // we pass a mutable reference to good_change
                         // allowing the borrower to change this reference.
} // s goes out of scope here is dropped

fn good_change(s: &mut String) { // s comes into scope and is a mutable reference
    s.push_str(" world"); // we mutate the mutable reference
} // s goes out of scope, but since this function does not own the reference nothing happens.


