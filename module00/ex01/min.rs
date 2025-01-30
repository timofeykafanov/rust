fn min(a: i32, b: i32) -> i32
{
    if a < b
    {
        a
    }
    else
    {
        b
    }
}

fn main()
{
    let x = 5;
    let y = 10;
    println!("The minimum of {} and {} is {}", x, y, min(x, y));
}