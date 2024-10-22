fn main() {
    // 변수와 가변성
    // let x = 5; // cannot assign twice to immutable variable
    let mut x  = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is  {x}");

    // 상수
    // 러스트의 이름 짓기 관례에서 상수는 단어 사이에 밑줄을 사용하고 모든 글자를 대문자로 쓰는 것입니다.
    // 전체 프로그램에 하드코딩된 값에 상수로써 이름을 붙이는 것은 미래의 코드 관리자에게 그 값의 의미를 전달하는 데 유용합니다. 또한 나중에 업데이트될 하드코딩된 값을 단 한 군데에서 변경할 수 있게 해 줍니다.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
}