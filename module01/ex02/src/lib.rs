pub const fn color_name(color: &[u8; 3]) -> &'static str
{
    let [r, g, b] = *color;

    match (r, g, b)
    {
        (0, 0, 0) => "pure black",
        (255, 255, 255) => "pure white",
        (255, 0, 0) => "pure red",
        (0, 255, 0) => "pure green",
        (0, 0, 255) => "pure blue",
        (128, 128, 128) => "perfect grey",
        (rr, gg, bb) => match (rr < 31, gg < 31, bb < 31, rr > 128, gg <= 127, bb <= 127, rr <= 127, gg > 128, bb > 128)
        {
            (true, true, true, _, _, _, _, _, _) => "almost black",
            (_, _, _, true, true, true, _, _, _) => "redish",
            (_, _, _, _, _, true, true, true, _) => "greenish",
            (_, _, _, _, true, _, true, _, true) => "blueish",
            _ => "unknown",
        },
    }
}



// #[cfg(test)]
// #[test]
// fn test_lifetimes() {
//     let name_of_the_best_color;

//     {
//         let the_best_color = [42, 42, 42];
//         name_of_the_best_color = color_name(&the_best_color);
//     }

//     assert_eq!(name_of_the_best_color, "unknown");
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_color_name() {
//         assert_eq!(color_name(&[42, 42, 42]), "unknown");
//         assert_eq!(color_name(&[0, 0, 0]), "pure black");
//         assert_eq!(color_name(&[255, 255, 255]), "pure white");
//         assert_eq!(color_name(&[255, 0, 0]), "pure red");
//         assert_eq!(color_name(&[0, 255, 0]), "pure green");
//         assert_eq!(color_name(&[0, 0, 255]), "pure blue");
//         assert_eq!(color_name(&[128, 128, 128]), "perfect grey");
//         assert_eq!(color_name(&[30, 30, 30]), "almost black");
//         assert_eq!(color_name(&[200, 100, 100]), "redish");
//         assert_eq!(color_name(&[100, 200, 100]), "greenish");
//         assert_eq!(color_name(&[100, 100, 200]), "blueish");
//     }
// }
