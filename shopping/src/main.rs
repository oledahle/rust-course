#[derive(PartialEq, Debug)]
enum Food {
WaterMelon, Milk, Coffee, PumpkinHead,
}

#[derive(PartialEq, Debug)]
struct ShoppingCart(Option<Food>);

#[derive(PartialEq, Debug)]
enum Answer {
    Yes,
    No,
    Maybe,
}

fn main() {
    println!("I want a bag for melon, {:?}", ShoppingCart(Some(Food::WaterMelon)));
}

fn want_bag(cart : ShoppingCart) -> Answer {
    match cart.0 {
        Some(Food::WaterMelon) | Some(Food::PumpkinHead) => Answer::Yes,
        Some(Food::Milk) => Answer::Maybe,
        _ => Answer::No
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_for_melon() {
        assert_eq!(Answer::Yes, want_bag(ShoppingCart(Some(Food::WaterMelon))));
    }

    #[test]
    fn nobag_for_coffee() {
        assert_eq!(Answer::No, want_bag(ShoppingCart(Some(Food::Coffee))));
    }

    #[test]
    fn maybebag_for_milk() {
        assert_eq!(Answer::Maybe, want_bag(ShoppingCart(Some(Food::Milk))));
    }
}