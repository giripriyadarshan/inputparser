[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link] [![crates.io license]][crates.io link]

# inputparser
Note: Thanks to [@Restioson](https://github.com/Restioson) and [@ThatsNoMoon](https://github.com/ThatsNoMoon) for helping me write the code

Takes terminal input in specified format

Instead of 
```rust
let mut var: String = String::new();
io::stdin().read_line(&mut var).unwrap();
let var: i32 = var.trim().parse().unwrap();
```
why not 
```rust
let var: i32 = inputparser::input(Def);
```
and it doesn't panic when wrong format is entered (when default arg [Def]).

## Usage
```toml
[dependencies]
inputparser = "0.1"
```

### Example
```rust
extern crate inputparser;
use crate::inputparser::input;
use crate::inputparser::ErHandle::*;

fn main() {
    let mut i: i32 = input(Def); //for Default continue message "Input not supported"
    let mut j: i32 = input(Pnc("Panic Message")); //for custom panic message
    let mut k: i32 = input(Msg("Continue Message")); //for custom continue message
    println!("{} {} {}", i, j, k);
}
```


[crates.io link]: https://crates.io/crates/inputparser
[crates.io version]: https://img.shields.io/crates/v/inputparser?style=for-the-badge
[Crates.io Downloads]: https://img.shields.io/crates/d/inputparser?style=for-the-badge
[crates.io license]: https://img.shields.io/crates/l/inputparser?style=for-the-badge