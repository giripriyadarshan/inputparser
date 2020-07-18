use std::io;
use std::str::FromStr;

pub fn input<T>() -> T
where
    T: FromStr,
{
    let mut input: String = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(input) = input.trim().parse::<T>() {
            break input;
        } else {
            println!("Input not supported");
        }
    }
}