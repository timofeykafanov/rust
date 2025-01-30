pub fn add(a: &i32, b: i32) -> i32
{
    a + b
}

pub fn add_assign(a: &mut i32, b: i32)
{
    *a += b;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add() {
//         let a = 5;
//         let b = 10;
//         assert_eq!(add(&a, b), 15);
//     }

//     #[test]
//     fn test_add_assign() {
//         let mut a = 5;
//         let b = 10;
//         add_assign(&mut a, b);
//         assert_eq!(a, 15);
//     }
// }
