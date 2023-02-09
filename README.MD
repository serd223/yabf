# Yet Another Brainfuck Interpreter

`yabf_rs` is a simple crate that can be used to embed Brainfuck programs into your Rust programs.

## Features

- Can parse and execute Brainfuck code.
- Highly customizable IO system.
- Allows you to inspect/modify data about your program during runtime.


## Running examples
### Prerequisites
- Install Rust from https://www.rust-lang.org/
- Install Git from https://git-scm.com/

```
git clone https://github.com/serd223/yabf
cd yabf
cargo run --example hello_world
```

## Adding `yabf_rs` to your project

```
cargo add yabf_rs
```


### Why `yabf_rs` instead of just `yabf`?
yabf was already taken...
