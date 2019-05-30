use std::env::{args, Args};
use std::iter::Skip;

fn main() {
    let args: Skip<Args> = args().skip(1);
    for arg in args {
        match arg.parse::<u32>() {
            Ok(intarg) => println!("{}", intarg),
            Err(e) => println!("fail: {} {}", arg, e)
        }
    }
}