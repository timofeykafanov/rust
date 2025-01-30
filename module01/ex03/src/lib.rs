pub fn largest_group<'a>(_haystack: &'a [u32], _needle: &[u32]) -> &'a [u32] {
    if _needle.is_empty() {
        return &[];
    }

    let mut max_len = 0;
    let mut max_slice: &'a [u32] = &[];

    for start in 0.._haystack.len() {
        for end in start.._haystack.len() {
            let current_slice = &_haystack[start..=end];
            if contains_all(current_slice, _needle) {
                let current_len = current_slice.len();
                if current_len > max_len {
                    max_len = current_len;
                    max_slice = current_slice;
                }
            }
        }
    }
    if max_len < _needle.len() {
        return &[];
    }
    max_slice
}

fn contains_all(slice: &[u32], needle: &[u32]) -> bool {
    for &item in slice {
        if !needle.contains(&item) {
            return false;
        }
    }
    true
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_lifetimes() {
//         let haystack = [1, 2, 3, 2, 1];
//         let result;

//         {
//             let needle = [2, 3];
//             result = largest_group(&haystack, &needle);
//         }

//         assert_eq!(result, &[2, 3, 2]);
//     }

//     #[test]
//     fn test_largest_group() {
//         assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
//         assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
//         assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
//         assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
//     }
// }