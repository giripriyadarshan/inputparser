[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link]

# inputparser
Takes terminal input in specified format

Instead of 
```rust
let mut input:String = String::new();
io::stdin().read_line(&mut input).unwrap();
let input:i32=input.trim().parse().unwrap();
```
why not 
```rust
let input:i32= inputparser::inti32conv();
```

and it shows error and doesn't exit till proper input is entered

## Usage
```toml
[dependencies]
inputparser = "0.1"
```

### Example
```rust
fn main() {
  let i = inputparser::inti32conv();
  println!("{}",i);
}
```


[crates.io link]: https://crates.io/crates/inputparser
[crates.io version]: https://img.shields.io/crates/v/inputparser?style=for-the-badge
[Crates.io Downloads]: https://img.shields.io/crates/d/inputparser?style=for-the-badge
