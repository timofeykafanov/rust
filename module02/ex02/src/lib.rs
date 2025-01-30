#[derive(Debug, PartialEq)]
#[allow(dead_code)]

enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

#[allow(dead_code)]

impl PizzaStatus {
    pub fn from_delivery_time(ordered_days_ago: u32) -> Self {
        match ordered_days_ago {
            0..=1 => PizzaStatus::Ordered,
            2..=6 => PizzaStatus::Cooking,
            7..=9 => PizzaStatus::Cooked,
            10..=16 => PizzaStatus::Delivering,
            _ => PizzaStatus::Delivered,
        }
    }
    pub fn get_delivery_time_in_days(&self) -> u32 {
        match self {
            PizzaStatus::Ordered => 17,
            PizzaStatus::Cooking => 15,
            PizzaStatus::Cooked => 10,
            PizzaStatus::Delivering => 7,
            PizzaStatus::Delivered => 0,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from_delivery_time() {
//         assert_eq!(PizzaStatus::from_delivery_time(1), PizzaStatus::Ordered);
//         assert_eq!(PizzaStatus::from_delivery_time(4), PizzaStatus::Cooking);
//         assert_eq!(PizzaStatus::from_delivery_time(8), PizzaStatus::Cooked);
//         assert_eq!(PizzaStatus::from_delivery_time(14), PizzaStatus::Delivering);
//         assert_eq!(PizzaStatus::from_delivery_time(17), PizzaStatus::Delivered);
//     }

//     #[test]
//     fn test_get_delivery_time_in_days() {
//         assert_eq!(PizzaStatus::Ordered.get_delivery_time_in_days(), 17);
//         assert_eq!(PizzaStatus::Cooking.get_delivery_time_in_days(), 15);
//         assert_eq!(PizzaStatus::Cooked.get_delivery_time_in_days(), 10);
//         assert_eq!(PizzaStatus::Delivering.get_delivery_time_in_days(), 8);
//         assert_eq!(PizzaStatus::Delivered.get_delivery_time_in_days(), 0);
//     }
// }
