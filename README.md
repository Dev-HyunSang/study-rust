# Study Rust
공식 문서인 [https://doc.rust-lang.org/book/title-page.html](https://doc.rust-lang.org/book/title-page.html)를 참고하며 공부하고 있습니다.

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
- [1.Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
  - [X] [Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
  - [X] [Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
  - [X] [Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [X] [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#programming-a-guessing-game)
- 