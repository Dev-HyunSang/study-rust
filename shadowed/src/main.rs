fn main() {
    // 섀도잉
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    
    let spaces = "   ";
    let spaces = spaces.len(); // help: if this is intentional, prefix it with an underscore: `_spaces
}
