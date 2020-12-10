use std::collections::HashSet;

use anyhow::Result as AnyResult;
use aoc_2020::lines;
use boolinator::Boolinator;

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Index(i32);

impl Index {
    /// Moves an index according to the instruction received.
    pub fn step(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Jmp(offset) => self.0 += offset,
            _ => self.0 += 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn load() -> AnyResult<Self> {
        fn parse(line: String) -> Option<Instruction> {
            let mut splitted = line.split(' ');

            let key = splitted.next()?;
            let val = splitted.next()?.parse::<i32>().ok()?;

            let inst = match key {
                "acc" => Instruction::Acc(val),
                "jmp" => Instruction::Jmp(val),
                "nop" => Instruction::Nop(val),
                _ => unreachable!("Unknown instruction"),
            };

            Some(inst)
        }

        let instructions = lines("files/day8/input.txt")?.filter_map(parse).collect();

        Ok(Self { instructions })
    }

    /// Start at the first instruction (0), if it's `acc` increase the value of
    /// the accumulator. Each processed instruction index is stored in a set,
    /// whenever we find reach an already executed instruction and if `avoid_loop`
    /// is true we end the program and return the value of the accumulator.
    fn run(&mut self, avoid_loop: bool) -> i32 {
        let mut counter = Index(0);
        let mut accumulator = 0i32;
        let mut executed = avoid_loop.as_some_from(HashSet::new);

        while let Some(instruction) = self.instructions.get(counter.0 as usize) {
            if let Some(executed) = executed.as_mut() {
                if executed.contains(&counter) {
                    return accumulator;
                }

                executed.insert(counter);
            }

            if let Instruction::Acc(val) = instruction {
                accumulator += val;
            }

            counter.step(instruction);
        }

        accumulator
    }

    /// Change one `nop` to `jmp`, or one `jmp` to `nop` in order to fix an
    /// infinite loop.
    fn fix(&mut self) -> &mut Self {
        let fix_idx = FixLoop::new(self.clone()).get_fix_idx();

        if let Some(fix_idx) = fix_idx {
            if let Some(instruction) = self.instructions.get_mut(fix_idx) {
                match instruction {
                    Instruction::Nop(val) => *instruction = Instruction::Jmp(*val),
                    Instruction::Jmp(val) => *instruction = Instruction::Nop(*val),
                    _ => unreachable!(),
                }
            }
        }

        self
    }
}

/// Used to fix the program infinite loop.
#[derive(Debug, Clone)]
struct FixLoop {
    program: Program,
}

impl FixLoop {
    fn new(program: Program) -> Self {
        Self { program }
    }

    /// Runs the program starting from one specific point and checks whether
    /// we find a loop.
    fn still_loop(&self, mut counter: Index, mut executed: HashSet<Index>) -> bool {
        while let Some(instruction) = self.program.instructions.get(counter.0 as usize) {
            if executed.contains(&counter) {
                return false;
            }

            executed.insert(counter);
            counter.step(instruction);
        }

        true
    }

    /// Returns the index, if there's one, of the instruction to change in order to fix the
    /// infinite loop.
    fn get_fix_idx(&mut self) -> Option<usize> {
        let mut counter = Index(0);
        let mut executed = HashSet::new();

        // Change in-place one instruction an simulate with `still_loop`, if it
        // doesn't loop anymore we found the index we need to change.
        while let Some(instruction) = self.program.instructions.get(counter.0 as usize).cloned() {
            match instruction {
                Instruction::Nop(val) => {
                    self.program.instructions[counter.0 as usize] = Instruction::Jmp(val);
                    if !self.still_loop(counter, executed.clone()) {
                        self.program.instructions[counter.0 as usize] = Instruction::Nop(val);
                    } else {
                        return Some(counter.0 as usize);
                    }
                }

                Instruction::Jmp(val) => {
                    self.program.instructions[counter.0 as usize] = Instruction::Nop(val);
                    if !self.still_loop(counter, executed.clone()) {
                        self.program.instructions[counter.0 as usize] = Instruction::Jmp(val);
                    } else {
                        return Some(counter.0 as usize);
                    }
                }

                _ => {}
            }

            executed.insert(counter);
            counter.step(&instruction);
        }

        None
    }
}

fn main() -> AnyResult<()> {
    let mut program = Program::load()?;

    println!("Day 8, Part 1: {}", program.run(true));
    println!("Day 8, Part 2: {}", program.fix().run(false));

    Ok(())
}
