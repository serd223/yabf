# Yet Another Brainfuck Interpreter

`yabf_rs` is a simple crate that can be used to embed Brainfuck programs into your Rust programs.
[Documentation on docs.rs](https://docs.rs/yabf_rs/)


## Features

- Can parse and execute Brainfuck code.
- Highly customizable IO system.
- Allows you to inspect/modify data about your program during runtime.

## Basic example

The following program outputs the letter 'H' to the terminal.
```rs
use yabf_rs::*;
fn main() {
    let program = Program::from(">++++++++[<+++++++++>-]<.");
    let mut bf: BfInstance<256> = BfInstance::from(program);
    bf.run(&mut default_input_source, &mut default_flush);
}
```

## More examples

For more examples, look at the [examples](https://github.com/serd223/yabf/tree/master/examples) directory in the repository.

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
