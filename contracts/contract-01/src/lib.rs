pub struct Counter {
    pub value: u32,
}

impl Counter {
    /// Creates a new instance. `value` starts at zero.
    pub fn new() -> Self {
        Counter { value: 0 }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn decrement(&mut self) {
        self.value -= 1;
    }

    pub fn set(&mut self, new_value: u32) {
        self.value = new_value;
    }

    pub fn print_value(&self) {
        println!("{}", self.value);
    }
}

fn main() {
    let mut counter = Counter::new();
    counter.print_value();
    counter.increment();
    counter.print_value();
    counter.decrement();
    counter.print_value();
    counter.set(2);
    counter.print_value();
}
