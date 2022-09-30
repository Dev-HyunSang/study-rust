# Study Rust
["프로그래밍 언어 러스트를 배웁시다!"](https://youtu.be/W9DO6m8JSSs)를 통해서 Rust를 공부하고 있습니다.

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
- [X] [프로그래밍 언어 러스트를 배웁시다! 001 Easy Rust in Korean: Intro](https://youtu.be/W9DO6m8JSSs)
- [X] [프로그래밍 언어 러스트를 배웁시다! 002 Easy Rust in Korean: Comments](https://youtu.be/x7GlQjh2aSw)
- [X] [프로그래밍 언어 러스트를 배웁시다! 003 Easy Rust in Korean: Integers](https://youtu.be/dEMYR99YIao)