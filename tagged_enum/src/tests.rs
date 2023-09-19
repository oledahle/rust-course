use crate::tagged::*;

#[test]
fn someday_is_weekend() {
    let day = Day::Saturday;
    assert_eq!(true, day.is_weekend());
}

#[test]
fn monday_is_not_weekend() {
    let day = Day::Monday;
    assert_eq!(false, day.is_weekend());
}

#[test]
fn saturday_is_weekend() {
    let day = Day::Saturday;
    assert_eq!(true, day.is_weekend());
}

#[test]
fn day_is_not_weekend() {
    let day = Day::Monday;
    assert_eq!(false, day.is_weekend());
}

#[test]
fn is_workday_description() {
    let workday = DaySpecifier::Weekday(Day::Tuesday);
    assert_eq!("Is work day: true", workday.description());
}

#[test]
fn is_weekend_description() {
    let workday = DaySpecifier::Weekday(Day::Saturday);
    assert_eq!("Is work day: false", workday.description());
}

#[test]
fn is_date_description() {
    let date = DaySpecifier::Date(Date {
        year: 2021,
        month: 10,
        day: 17,
    });
    assert_eq!(
        "Date { year: 2021, month: 10, day: 17 }",
        date.description()
    );
}
