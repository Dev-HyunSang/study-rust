fn five() -> i32 {
    5
}

fn main() {
    // 상수는 const, 변수는, let
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // `MAX_POINTS` is never used
    // const MAX_POINTS:u32 = 100_000;

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // let spaces = "   ";
    // let mut spaces = "   ";
    // let spaces = spaces.len();

    // let guess: u32 = "42".parse().expect("Not a number!");

    // 부동소수점 숫자는 IEEE-754 표준에 따라 표현됩니다. f32 타입은 1배수의 정밀도인 부동소수점이고, f64는 2배수의 정밀도인 부동소수점입니다.
    // http://www.tcpschool.com/cpp/cpp_datatype_floatingPointNumber / https://steemit.com/kr/@modolee/floating-point
    // let x = 2.0; // f64
    // let y: f32 = 3.0; //f32

    println!("Hello,world!");
    another_function();
    another_function2(5);
    another_function3(5, 6);

    // let y = 6;
    // let x = (let y = 6);
    let x = 5;
    let y = {
        let x = 3;
        x + 1;
    };

    println!("The value of y is: {:#?}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x =plus_one(5);

    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}