
#![allow(arithmetic_overflow)]

fn main()
{
    let a: u8 = 255;
    let b: u8 = 1;
    let result = a + b;
    println!("{}u8 + {}u8 == {}", a, b, result);
}