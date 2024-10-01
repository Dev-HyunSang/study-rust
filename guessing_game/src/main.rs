use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // expect를 호출하지 않으면 컴파일은 되지만 경고가 나타남.

    println!("You guessed: {guess}");
}
