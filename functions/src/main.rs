fn main() {
    another_function(5, 6);
    let x = 14;

    let y = {
        let x = 3;
        println!("The value of x is: {}", x);
        x + 1 // an expression does not include ending semicolons; adding a semicolon will make it a statement
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);

    let a = array()[1];

    println!("The value of a is: {}", a);

    let tup = tup();

    println!("The last index value is: {}", tup.3);

    outer_fun();
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn array() -> [i32; 5] {
    [0,2,1,8,4]
}

fn tup() -> (i32, i32, i32, char) {
    (1,2,3,'a')
}

fn outer_fun() {
    fn inner_fun() {
        println!("Inner function");
    }
    inner_fun();
    println!("Outer function");
}
