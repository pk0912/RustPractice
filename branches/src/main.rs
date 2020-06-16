fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // it works without semicolon as well
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("{}", fibonacci(184)); // maximum limit

    println!("{}", factorial(33)); // maximum limit
}

fn fibonacci(n: i32) -> i128  {
    fn handler(x: i32, prev: i128, second_prev: i128) -> i128  {
      if x <= 2 {
          prev
      }
      else {
          handler(x-1, prev + second_prev, prev)
      }
    }
    handler(n, 1, 1)
  }

fn factorial(n: i128) -> i128 {
    fn handler(x: i128, accumulator: i128) -> i128 {
        if x <= 1 {
            accumulator
        }
        else {
            handler(x-1, x * accumulator)
        }
    }
    handler(n, 1)
}