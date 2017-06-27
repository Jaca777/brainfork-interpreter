use machine::*;
use std::io;

enum Instruction {
    Forward,
    Backward,
    Increment,
    Decrement,
    Read,
    Write,
    JumpToIf { ptr: usize, condition: JumpCondition },
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
    pub fn step(&mut self) -> Result<(), &'static str> {
        let state_result: Result<(), &'static str> = match self.machine.state() {
            MachineState::Idle => Err("Machine is idle - cannot execute a step."),
            MachineState::Interrupted {cause} => Err("Machine runtime has been interrupted, cannot step forward"),
            MachineState::Running => Ok(())
        };
        self.instructionPtr += 1;
        let insn = self.program.0.get(self.instructionPtr);
        match insn {
            Some(insn) => {
                let final_state = state_result.and_then(|_| self.executeInsn(*insn));
                if let Err(e) = final_state { self.machine.interrupt(e) }
                final_state
            }
            None => Err("Cannot step forward, program has reached its end.")
        }
    }

    fn executeInsn(&mut self, insn: Instruction) -> Result<(), &'static str> {
        match insn {
            Forward => self.machine.forward(),
            Backward => self.machine.backward(),
            Increment => {
                self.machine.increment();
                Ok(())
            }
            Decrement => {
                self.machine.decrement();
                Ok(())
            }
            Read => {
                let mut input = String::new();
                let result = io::stdin().read_line(&mut input)
                    .map_err(|_| "Failed to read line")
                    .and_then(|_| input.parse().map_err(|_| "Failed to parse input"));
                if let Ok(value) = result {
                    self.machine.set(value);
                }
                result.map(|_| ())
            }
            Write => {
                println!("{}", self.machine.access());
                Ok(())
            }
        }
    }
}
