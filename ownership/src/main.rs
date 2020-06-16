fn main() {
    let s = String::from("hello");

    println!("{}", s);

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1; // This is not shallow copying; it looks like that but it is a move operation and not a copy operation 

    // println!("{}, world!", s1); // This will give compile time error: value used here after move
    println!("{}", s2);

    let a = 5;
    let b = a;
    println!("{}, {}, {}", a, b, a == b);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}, {}", s1, s2, s1 == s2);

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // println!(s');                // This will give compile time error as value of s has been moved to some_string of takes_ownership function and is no longer valid.

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    println!("s1 = {}", s1);

    let s2 = String::from("hello");     // s2 comes into scope

    println!("s2 = {}", s2);

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    println!("s3 = {}", s3);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");

    let len = calculate_length_using_ref(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let mut s = String::from("hello");

    let r1 = &mut s;

    // let r2 = &mut s; // Compile time error: cannot borrow `s` as mutable more than once at a time

    println!("{}", r1);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        change(r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    change(r2);

    println!("{}", s);    

    let mut s = String::from("hello");

    let r1 = &s; // no problem 
    let r2 = &s; // no problem as its an immutable reference and above one is immutable as well
    // let r3 = &mut s; // BIG PROBLEM: Compile Time Error: We cannot have a mutable reference while we have an immutable one. 

    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // let reference_to_nothing = dangle(); // Compile time error: explained in function declaration below
    let reference_to_something = no_dangle();
    println!("{}", reference_to_something);

    let s = String::from("hello");

    let slice = &s[0..2];
    println!("Slice : {}", slice);
    let slice = &s[..2];
    println!("Slice : {}", slice);

    let len = s.len();

    let slice = &s[3..len];
    println!("Slice : {}", slice);
    let slice = &s[3..];
    println!("Slice : {}", slice);

    let slice = &s[0..len];
    println!("Slice : {}", slice);
    let slice = &s[..];
    println!("Slice : {}", slice);

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // compile time error: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("first word : {}", word);

    s.clear(); // Here it will work as the immutable reference of s in variable word is not getting used anywhere else in the program now

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3]; // return type of slice here is &[i32]
    println!("{},{}", slice[0], slice[1]);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
  // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_using_ref(s: &String) -> usize { // s is a reference to a String
  // s.push_str(", world") // This will give compile time error as this is an immutable reference
  s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
  // This is known as Borrowing.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Following function will give compile time error
// fn dangle() -> &String { // dangle returns a reference to a String
//   let s = String::from("hello"); // s is a new String

//   &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!

// The solution for above problem is to return the String directly:
fn no_dangle() -> String {
  let s = String::from("hello");

  s
}

// &str: it’s a slice pointing to that specific point of the binary.
// fn first_word(s: &String) -> &str { // A better way to write the signature of this function is given below: 
fn first_word(s: &str) -> &str {
  // If we have a string slice, we can pass that directly. 
  // If we have a String, we can pass a slice of the entire String. 
  // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}
