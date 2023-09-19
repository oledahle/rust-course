    #[derive(PartialEq, Debug)]
    #[allow(dead_code)]
    pub enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        pub fn is_weekend(self) -> bool {
            if self == Day::Saturday || self == Day::Sunday {
                true
            } else {
                false
            }
        }
    }

    #[allow(dead_code)]
    #[derive(PartialEq, Debug)]
    pub struct Date {
        pub year: u16,
        pub month: u8,
        pub day: u8,
    }

    #[derive(PartialEq, Debug)]
    pub enum DaySpecifier {
        Weekday(Day),
        Date(Date),
    }

    impl DaySpecifier {
        pub fn description(self) -> String {
            match self {
                DaySpecifier::Weekday(day) => {
                    if Day::is_weekend(day) {
                        return "Is work day: false".into();
                    } else {
                        return "Is work day: true".into();
                    }
                },
                DaySpecifier::Date(day) => {
                    format!("{:?}", day)
                }
            }
        }
    }
