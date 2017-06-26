use machine::Machine;
use machine::Navigable;
use machine::Data;

enum Instruction {
    Forward,
    Backward,
    Increment,
    Decrement,
    Read,
    Write,
    JumpToIf { ptr: usize, condition:  JumpCondition},
    Fork
}

enum JumpCondition {
    Eq0,
    Neq0
}

struct Program([Instruction]);

struct ProgramRuntime {
    machine: Machine,
    instructionPtr: usize,
    program: Program
}

impl ProgramRuntime {
    pub fn step(&self) {
        let insn = self.program.0[self.instructionPtr];
        match insn {
            Forward => self.submit(self.machine.forward()),
            Backward => self.submit(self.machine.backward()),
            Increment => self.submit(self.machine.increment()),
            Decrement => self.submit(self.machine.decrement()),
            Read =>
            Write,
        }
    }

    fn submit(&self, stepResult: Result<(), &'static str> ) -> Result<(), &'static str> {
        match stepResult {
            Err(cause) => self.machine.interrupt(cause),
            Ok(_) => () //everything is ok
        };
        stepResult
    }
}

fn bind(machine: Machine, program: Program) -> ProgramRuntime {

}
