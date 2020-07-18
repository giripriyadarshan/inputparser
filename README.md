[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link] [![crates.io license]][crates.io link]

# inputparser
Takes terminal input in specified format

Thanks to [@Restioson](https://github.com/Restioson) and [@ThatsNoMoon](https://github.com/ThatsNoMoon) for helping to write this crate

Instead of 
```rust
let mut input: String = String::new();
io::stdin().read_line(&mut input).unwrap();
let input: i32 = input.trim().parse().unwrap();
```
why not 
```rust
let input: i32 = inputparser::input();
```
and it doesn't panic when wrong format is entered.

## Usage
```toml
[dependencies]
inputparser = "0.1"
```

### Example
Basic
```rust
fn main() {
    let i: i32 = inputparser::input();
    println!("{}", i);
}
```
Effective
```rust
extern crate inputparser;
use crate::inputparser::input;

fn main() {
    let mut i: i32 = input();
    println!("{}", i);
}
```


[crates.io link]: https://crates.io/crates/inputparser
[crates.io version]: https://img.shields.io/crates/v/inputparser?style=for-the-badge
[Crates.io Downloads]: https://img.shields.io/crates/d/inputparser?style=for-the-badge
[crates.io license]: https://img.shields.io/crates/l/inputparser?style=for-the-badge
