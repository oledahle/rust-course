    pub struct Counter {
        num: u32,
    }

    impl Counter {
        pub fn new() -> Self {
            Counter { num: 0 }
        }

        pub fn increment(&mut self) {
            self.num += 1;
        }

        pub fn print(&self) {
            println!("Counter: {}", self.num);
        }
    }
