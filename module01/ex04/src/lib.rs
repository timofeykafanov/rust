pub fn sort_boxes(boxes: &mut [[u32; 2]])
{
    let len = boxes.len();
    for _i in 0..len {
        for j in 0..len - 1 {
            if boxes[j][0] < boxes[j + 1][0] || (boxes[j][0] == boxes[j + 1][0] && boxes[j][1] < boxes[j + 1][1]) {
                boxes.swap(j, j + 1);
            }
        }
    }

    for i in 0..boxes.len() - 1 {
        if boxes[i][0] < boxes[i + 1][0] || boxes[i][1] < boxes[i + 1][1] {
            panic!("Boxes can not be sorted!");
        }
    }
    
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_sort_boxes() {
//         let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
//         sort_boxes(&mut boxes);
//         assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
//     }
// }
