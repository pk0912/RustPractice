fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    // let MAX_POINTS = "qwert"; // This will give compile time error : `MAX_POINTS` is interpreted as a constant, not a new binding

    println!("The Max Point is: {}", MAX_POINTS);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Lenght of spaces: {}", spaces);

    /* This will give compile time error : error[E0308]: mismatched types; expected `&str`, found `usize`
        let mut spaces = "   ";
        spaces = spaces.len();
    */

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';

    let z = 'ℤ';

    let heart_eyed_cat = '😻';

    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1); // optional type annotations 

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // destructuring

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("{}, {}, {}, {}", five_hundred, six_point_four, one, tup.2);

    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{}", a[0]);

    let a = [3; 5]; // to create an array that contains the same value for each element; 3 is the value and 5 is the length

    println!("{}, {}", a[0], a[4]);

    // println!("{}", a[5]) // Rust’s safety principles in action; When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the array length, Rust will panic.
}
