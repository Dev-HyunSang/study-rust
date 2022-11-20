fn main() {
    let number = 3;
    // let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // ERROR:expected `bool`, found integer
    // if number {
    //     println!("number was three");
    // }

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
    let number = if condition {
        5
    } else {
        6
        // expected integer, found `&str
        // "six"
    };

    println!("The value of number is: {}", number);

    // loop {
    //     println!("again!");
    // }
    
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

   println!("LIFTOFF!!!");

   let a = [10, 20, 30, 40, 50];

   for element in a.iter() {
    println!("the value is: {}", element);
   }

   for number in (1..4).rev() {
    println!("{}!", number);
   }

   println!("LIFTOFF!!!");
}