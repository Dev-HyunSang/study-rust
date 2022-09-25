# Study Rust

[The Rust Programming Language](https://rinthel.github.io/rust-lang-book-ko/foreword.html)를 통해서 Rust를 공부하고 있습니다.

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
- [X] [시작하기](https://rinthel.github.io/rust-lang-book-ko/ch01-00-getting-started.html)
    - [X] [설치하기](https://rinthel.github.io/rust-lang-book-ko/ch01-01-installation.html)
    - [X] [Hello, World!](https://rinthel.github.io/rust-lang-book-ko/ch01-02-hello-world.html)
    - [X] [Hello, Cargo!](https://rinthel.github.io/rust-lang-book-ko/ch01-03-hello-cargo.html)