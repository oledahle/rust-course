mod counter;
use counter::Counter;

fn main() {
    let mut counter = Counter::new();
    counter.increment();
    counter.print();
}