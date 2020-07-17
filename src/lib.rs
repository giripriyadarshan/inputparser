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