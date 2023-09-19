#[derive(PartialEq, Debug)]
enum Action {
    Jump,
    Fall,
    BreakLeg,
}

fn main() {
    println!("Hello, world!");
}

fn fearless_action(a :Action) -> Action {
    match a {
        (Action::Jump) => Action::Fall,
        (Action::Fall) => Action::BreakLeg,
        (Action::BreakLeg) => Action::BreakLeg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_to_fall() {
        assert_eq!(Action::Fall, fearless_action(Action::Jump));
    }

    #[test]
    fn fall_to_breakleg() {
        assert_eq!(Action::BreakLeg, fearless_action(Action::Fall));
    }

    #[test]
    fn breakleg_to_breakleg() {
        assert_eq!(Action::BreakLeg, fearless_action(Action::BreakLeg));
    }
}