pub fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_min() {
//         let x = 5;
//         let y = 2;
//         assert_eq!(min(&x, &y), &y);
//         assert_eq!(min(&y, &x), &y);
//     }
// }
