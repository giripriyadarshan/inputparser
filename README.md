[![crates.io version]][crates.io link] [![Crates.io Downloads]][crates.io link]]

# inputparser
Takes terminal input in specified format

## Usage
```toml
[dependencies]
inputparser = "0.1"
```

### Example
```rust
fn main() {
  let i = inputparser::int32conv();
  println!("{}",i);
}
```


[crates.io link]: https://crates.io/crates/inputparser
[crates.io version]: https://img.shields.io/crates/v/inputparser?style=for-the-badge
[Crates.io Downloads]: https://img.shields.io/crates/d/inputparser?style=for-the-badge
