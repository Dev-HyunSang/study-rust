# Study Rust
공식 문서인 [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)의 한국어 번역판인 [The Rust Programming Language](https://rinthel.github.io/rust-lang-book-ko/foreword.html)를 참고하여 공부하고 있습니다.

## Getted Starting
### Installation
```sh
$ brew install ruby
# OR
$ curl https://sh.rustup.rs -sSf | sh
Rust is installed now. Great!
# OR
$ brew install rust
$ source $HOME/.cargo/env
# OR
$ export PATH="$HOME/.cargo/bin:$PATH"
```

### Update & Delete
```shell
$ rustup update
$ rustup self uninstall
```

## Types
- **Primitive Type(데이터 타입):**
  - Boolean - `bool`
  - Numeric - integer and float
  - Textual - `char` and `str`
  - Never - `!` - a type with no values
- **Sequence types:**
  - Tuple
  - Array
  - Slice
- **User-defined types:**
  - Struct
  - Enum
  - Union
- **Function types:**
  - Functions
  - Closures
- **Pointer types:**
  - References
  - Raw pointers
  - Function pointers
- **Trait types:**
  - Trait objects
  - Impl trait

## ToDo
- [1.시작하기](https://rinthel.github.io/rust-lang-book-ko/ch01-00-getting-started.html)
  - [X] [설치하기](https://rinthel.github.io/rust-lang-book-ko/ch01-01-installation.html)
  - [X] [Hello, World!](https://rinthel.github.io/rust-lang-book-ko/ch01-02-hello-world.html)
  - [X] [Hello, Cargo!](https://rinthel.github.io/rust-lang-book-ko/ch01-03-hello-cargo.html)
- [X] [2.추리 게임 튜토리얼](https://rinthel.github.io/rust-lang-book-ko/ch02-00-guessing-game-tutorial.html)
- [X] [3.보편적인 프로그래밍 개념](https://rinthel.github.io/rust-lang-book-ko/ch03-00-common-programming-concepts.html) - 2022.11.08 완료
  - [X] [3.1.변수와 가변성](https://rinthel.github.io/rust-lang-book-ko/ch03-01-variables-and-mutability.html) - 2022.11.08 완료
  - [X] [3.2.데이터 타입들](https://rinthel.github.io/rust-lang-book-ko/ch03-02-data-types.html) - 2022.11.08 완료
  - [X] [3.3.함수 동작 원리](https://rinthel.github.io/rust-lang-book-ko/ch03-03-how-functions-work.html) - 2022.11.08 완료
  - [X] [3.4.주석](https://rinthel.github.io/rust-lang-book-ko/ch03-04-comments.html) - 2022.11.08 완료
  - [X] [3.5.제어문](https://rinthel.github.io/rust-lang-book-ko/ch03-05-control-flow.html) - 2022.11.21 완료

## Error Solution
### `()` doesn't implement `std::fmt::Display`
```shell
error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> src/main.rs:45:39
   |
45 |     println!("The value of y is: {}", y);
   |                                       ^ `()` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `()`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```
```rs
let x = 5;
let y = {
  let x = 3;
  x + 1;
};
println!("The value of y is: {}", y);
```

**해결 방법:** `println!("The value of y is: {}", y);`으로 출력하고자 했지만 `println!("The value of y is: {:#?}", y);`으로 출력하면 됩니다. [Cannot be formatted with the default formatter](https://users.rust-lang.org/t/cannot-be-formatted-with-the-default-formatter/57043)를 참고하였습니다.