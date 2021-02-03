#[derive(Copy, Clone)]
pub struct ProgramCounter(u16);

impl ProgramCounter {
    pub fn new_with_value(val: u16) -> ProgramCounter {
        ProgramCounter(val)
    }

    pub fn set_to(&mut self, val: u16) -> () {
        self.0 = val
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn increment(&mut self) -> () {
        self.0 += 2
    }

    pub fn increment_if(&mut self, condition: bool) -> () {
        if condition {
            self.increment()
        }
    }

    pub fn decrement_if(&mut self, condition: bool) -> () {
        if condition {
            self.0 -= 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProgramCounter;

    #[test]
    fn program_counter_operates_correctly() {
        let mut pc = ProgramCounter::new_with_value(10);

        assert_eq!(pc.value(), 10);

        pc.set_to(11);
        assert_eq!(pc.value(), 11);

        pc.increment();
        assert_eq!(pc.value(), 13);

        pc.increment_if(true);
        assert_eq!(pc.value(), 15);

        pc.decrement_if(false);
        assert_eq!(pc.value(), 15);
    }
}
