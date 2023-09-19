mod tagged;
mod tests;

use tagged::*;

fn main() {
    let workday = DaySpecifier::Weekday(Day::Friday);
    println!("{}",workday.description());
}
