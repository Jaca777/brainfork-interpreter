// Author: Jaca777
// 26.06.2017
use std::result;

pub trait Navigable: Sized {
    fn forward(&mut self) -> Result<(), &'static str>;
    fn backward(&mut self) -> Result<(), &'static str>;
}

pub trait Data<T> {
    fn increment(&mut self);
    fn decrement(&mut self);
    fn access(&self) -> T;
}

pub enum MachineState {
    Idle,
    Running,
    Interrupted { cause: &'static str }
}

pub struct Machine {
    state: MachineState,
    pointer: usize,
    capacity: usize,
    tape: Vec<i8>
}

impl Machine {
    pub fn interrupt(&mut self, cause: &'static str) {
        self.state = MachineState::Interrupted { cause }
    }
}

impl Navigable for Machine {
    fn forward(&mut self) -> Result<(), &'static str> {
        if self.pointer + 1 < self.capacity {
            self.pointer += 1;
            Ok(())
        } else { Err("Pointer exceeds machine tape size.") }
    }

    fn backward(&mut self) -> Result<(), &'static str> {
        if self.pointer > 0 {
            self.pointer -= 1;
            Ok(())
        } else { Err("Pointer already points at a 0 index - unable to move it backwards.") }
    }
}

impl Data<i8> for Machine {
    fn increment(&mut self) {
        let curr_value = self.access();
        self.tape[self.pointer] = curr_value + 1;
    }

    fn decrement(&mut self) {
        let curr_value = self.access();
        self.tape[self.pointer] = curr_value + 1;
    }

    fn access(&self) -> i8 {
        self.tape[self.pointer]
    }
}
