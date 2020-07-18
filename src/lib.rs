use proc_macro::TokenStream;
use std::ffi::OsString;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::path::PathBuf;

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
pub fn intisizeconv() -> isize {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<isize>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn intusizeconv() -> usize {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<usize>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn osstringconv() -> OsString {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<OsString>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn ipv4conv() -> Ipv4Addr {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<Ipv4Addr>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn ipv6conv() -> Ipv6Addr {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<Ipv6Addr>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn socv4conv() -> SocketAddrV4 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<SocketAddrV4>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn socv6conv() -> SocketAddrV6 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<SocketAddrV6>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzeroi128conv() -> NonZeroI128 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroI128>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzeroi64conv() -> NonZeroI64 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroI64>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzeroi32conv() -> NonZeroI32 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroI132>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzeroi16conv() -> NonZeroI16 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroI16>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzeroi8conv() -> NonZeroI8 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroI8>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzeroisizeconv() -> NonZeroIsize {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroIsize>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzerou128conv() -> NonZeroU128 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroU128>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzerou64conv() -> NonZeroU64 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroU64>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzerou32conv() -> NonZeroU32 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroU32>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzerou16conv() -> NonZeroU16 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroU16>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzerou8conv() -> NonZeroU8 {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroU8>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn nzerousizeconv() -> NonZeroUsize {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<NonZeroUsize>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn pathbufconv() -> PathBuf {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<PathBuf>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn tokenstreamconv() -> TokenStream {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<TokenStream>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn charconv() -> char {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<char>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn ipaddrconv() -> IpAddr {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<IpAddr>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}
pub fn socaddrconv() -> SocketAddr {
    let mut s: String = String::new();
    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(s) = s.trim().parse::<SocketAddr>() {
            break s;
        } else {
            println!("Input not supported");
        }
    }
}