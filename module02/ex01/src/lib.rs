#[allow(dead_code)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        f32::sqrt(dx * dx + dy * dy)
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new() {
//         let p = Point::new(1.0, 2.0);
//         assert_eq!(p.x, 1.0);
//         assert_eq!(p.y, 2.0);
//     }

//     #[test]
//     fn test_zero() {
//         let p = Point::zero();
//         assert_eq!(p.x, 0.0);
//         assert_eq!(p.y, 0.0);
//     }

//     #[test]
//     fn test_distance() {
//         let p1 = Point::new(0.0, 0.0);
//         let p2 = Point::new(3.0, 4.0);
//         assert_eq!(p1.distance(&p2), 5.0);
//     }

//     #[test]
//     fn test_translate() {
//         let mut p = Point::new(1.0, 2.0);
//         p.translate(3.0, 4.0);
//         assert_eq!(p.x, 4.0);
//         assert_eq!(p.y, 6.0);
//     }
// }
