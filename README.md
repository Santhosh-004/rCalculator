# rCalculator
#### _A simple calculator made with rust_

#### Running from source:
- Install rust using [rustup](https://rust-lang.org/tools/install/)
- Clone this repo
- Run the command: `cargo run`

#### Usage:
- Enter expressions like `8/4/2` and you will get `1` as result
- Supports operators: `+`, `-`, `*`, `/` with proper mathematical precedence
- Example: `2+3*4` returns `14` (not `20`)

#### What it can be used for:
- Learning Rust programming language
- Basic arithmetic calculations
- Understanding expression parsing
- Example of clean Rust code structure

#### Shortcomings:
- No parentheses support `(1+2)*3`
- No negative number handling `3*-2`
- No spaces in input `2 + 3` won't work
- No error recovery - one error exits program
- Only supports basic operators `+`, `-`, `*`, `/`