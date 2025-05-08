pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new(start: u32) -> Self {
        Counter { value: start }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn increment(&mut self) -> u32 {
        self.value += 1;
        self.value()
    }
}