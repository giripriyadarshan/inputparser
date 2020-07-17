use std::io;
pub fn int128conv() -> i128 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i128>() {
            if s > 0 || s == 0 || s < 0 {
                break;
            }
        } else {
            println!("please enter numbers only, special characters not supported");
        }
    }
    let s: i128 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn int08conv() -> i8 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i8>() {
            if s > 0 || s == 0 || s < 0 {
                break;
            }
        } else {
            println!("please enter numbers only, special characters not supported");
        }
    }
    let s: i8 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn int16conv() -> i16 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i16>() {
            if s > 0 || s == 0 || s < 0 {
                break;
            }
        } else {
            println!("please enter numbers only, special characters not supported");
        }
    }
    let s: i16 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn int32conv() -> i32 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i32>() {
            if s > 0 || s == 0 || s < 0 {
                break;
            }
        } else {
            println!("please enter numbers only, special characters not supported");
        }
    }
    let s: i32 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn int64conv() -> i64 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<i64>() {
            if s > 0 || s == 0 || s < 0 {
                break;
            }
        } else {
            println!("please enter numbers only, special characters not supported");
        }
    }
    let s: i64 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn flo64conv() -> f64 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<f64>() {
            if s > 0.0 || s == 0.0 || s < 0.0 {
                break;
            }
        } else {
            println!("please enter floating integers only, special characters not supported");
        }
    }
    let s: f64 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn flo32conv() -> f32 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<f32>() {
            if s > 0.0 || s == 0.0 || s < 0.0 {
                break;
            }
        } else {
            println!("please enter floating integers only, special characters not supported");
        }
    }
    let s: f32 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn stringinput() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).unwrap();
    return s.trim().to_string();
}
pub fn boolconv() -> bool {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<bool>() {
            if s == true || s == false {
                break;
            }
        } else {
            println!("Please enter true or false");
        }
    }
    let s: bool = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn intu128conv() -> u128 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u128>() {
            if s > 0 || s == 0 {
                break;
            }
        } else {
            println!("please enter positive u128 numbers only, special characters not supported");
        }
    }
    let s: u128 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn intu64conv() -> u64 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u64>() {
            if s > 0 || s == 0 {
                break;
            }
        } else {
            println!("please enter positive u64 numbers only, special characters not supported");
        }
    }
    let s: u64 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn intu32conv() -> u32 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u32>() {
            if s > 0 || s == 0 {
                break;
            }
        } else {
            println!("please enter positive u32 numbers only, special characters not supported");
        }
    }
    let s: u32 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn intu16conv() -> u16 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u16>() {
            if s > 0 || s == 0 {
                break;
            }
        } else {
            println!("please enter positive u16 numbers only, special characters not supported");
        }
    }
    let s: u16 = s.to_string().trim().parse().unwrap();
    return s;
}
pub fn intu8conv() -> u8 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<u8>() {
            if s > 0 || s == 0 {
                break;
            }
        } else {
            println!("please enter positive u16 numbers only, special characters not supported");
        }
    }
    let s: u8 = s.to_string().trim().parse().unwrap();
    return s;
}