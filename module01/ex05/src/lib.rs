pub fn deduplicate(list: &mut Vec<i32>)
{
    let mut i = 0;
    while i < list.len()
    {
        let mut j = i + 1;
        while j < list.len()
        {
            if list[i] == list[j]
            {
                list.remove(j);
            }
            else
            {
                j += 1;
            }
        }
        i += 1;
    }
}

// #[cfg(test)]
// mod tests
// {
//     use super::*;

//     #[test]
//     fn test_deduplicate()
//     {
//         let mut v = vec![1, 100, 2, 2, 3, 2, 4, 3, 5];
//         deduplicate(&mut v);
//         assert_eq!(v, [1, 100, 2, 3, 4, 5]);
//     }
// }
