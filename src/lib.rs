use std::io;
pub fn inti128conv() -> i128 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i128>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn inti8conv() -> i8 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i8>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn inti16conv() -> i16 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i16>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn inti32conv() -> i32 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i32>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn inti64conv() -> i64 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i64>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn flo64conv() -> f64 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<f64>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn flo32conv() -> f32 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<f32>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn stringinput() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}
pub fn boolconv() -> bool {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<bool>() {
            break s;
        } else {
            println!("Please enter true or false");
        }
    }
}
pub fn intu128conv() -> u128 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u128>() {
            break s;
        } else {
            println!("Please input numbers between 0 and {}", u128::MAX);
        }
    }
}
pub fn intu64conv() -> u64 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u64>() {
            break s;
        } else {
            println!("Please input numbers between 0 and {}", u64::MAX);
        }
    }
}
pub fn intu32conv() -> u32 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u32>() {
            break s;
        } else {
            println!("Please input numbers between 0 and {}", u32::MAX);
        }
    }
}
pub fn intu16conv() -> u16 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u16>() {
            break s;
        } else {
            println!("Please input numbers between 0 and {}", u16::MAX);
        }
    }
}
pub fn intu8conv() -> u8 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u8>() {
            break s;
        } else {
            println!("Please input numbers between 0 and {}", u8::MAX);
        }
    }
}
