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
