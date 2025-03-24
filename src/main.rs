use bracket_lib::prelude::*;

fn main() {
    println!("Hello, world!");
    base23(5);
}

fn base23(x: i32) -> i32 {
    print!("这是一个32位的值{x}");
    return x;
}