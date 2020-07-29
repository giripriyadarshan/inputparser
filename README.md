[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link] [![crates.io license]][crates.io link] [![Discord image]][Discord server]

# inputparser
Note: Thanks to [@Restioson](https://github.com/Restioson), [@ThatsNoMoon](https://github.com/ThatsNoMoon) and [@kangalioo](https://github.com/kangalioo) for helping me write the code

Now Rust inputs are almost as simple as **Python**

Terminal inputs are now easier than ever. Replacing over 5 lines of codes with just 1 function.

Supports **all formats** that FromStr Supports

Instead of 
```rust
let mut var: String = String::new();
io::stdin().read_line(&mut var).unwrap();
let var: i32 = var.trim().parse().unwrap();
```
why not 
```rust
let var: i32 = input(Def);
```
and it doesn't panic when wrong format is entered (when default arg [Def]).

Or you can choose to make it panic too.

## Usage
```toml
[dependencies]
inputparser = "0.1"
```

### Example
```rust
extern crate inputparser;
use inputparsertest::{input, input_w_msg, inputfn, ErHandle::*};

fn main() {
    //for Default continue message "Input not supported" when Err
    let i: i32 = input(Def);

    //for custom panic message when Err
    let j: f64 = input(Pnc("Panic Message"));

    //for custom continue message when Err
    let k: u128 = input(Msg("Continue Message"));

    //for custom loop message and continue/error message
    let l: isize = input_w_msg("Enter the number",Msg("Please enter valid number"));

    //for more Rust way for handling the error
    let m: usize = inputfn(|| /*use panic if required*/ println!("Continue Error Message"));
 
    println!("{} {} {} {} {}", i, j, k, l, m);
}
```


[crates.io link]: https://crates.io/crates/inputparser
[crates.io version]: https://img.shields.io/crates/v/inputparser?style=for-the-badge
[Crates.io Downloads]: https://img.shields.io/crates/d/inputparser?style=for-the-badge
[crates.io license]: https://img.shields.io/crates/l/inputparser?style=for-the-badge
[Discord image]: https://img.shields.io/discord/705030135312547912?color=blue&label=Support%20Server&style=for-the-badge
[Discord server]: https://discord.gg/F69SCTT