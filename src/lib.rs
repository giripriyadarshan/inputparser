use std::io;
use std::str::FromStr;
use ErHandle::*;

pub enum ErHandle<T> {
    Pnc(T),
    Msg(T),
    Def,
}

pub fn input<T>(e: ErHandle<&str>) -> T
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
            match e {
                Pnc(e) => panic!("{}", e),
                Msg(e) => println!("{}", e),
                Def => println!("Input not supported"),
            }
        }
    }
}

pub fn input_w_msg<T>(a: &str, e: ErHandle<&str>) -> T
where
    T: FromStr,
{
    let mut input: String = String::new();
    loop {
        println!("{}", a);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(input) = input.trim().parse::<T>() {
            break input;
        } else {
            match e {
                Pnc(e) => panic!("{}", e),
                Msg(e) => println!("{}", e),
                Def => println!("Input not supported"),
            }
        }
    }
}