use std::collections::HashMap;

use anyhow::{anyhow, Result as AnyResult};
use aoc_2020::lines;
use boolinator::Boolinator;

#[derive(Debug, Clone, Copy)]
enum BitMask {
    None,
    One,
    Zero,
}

#[derive(Debug, Clone)]
struct Mask {
    inner: Vec<BitMask>,
}

impl<S: AsRef<str>> From<S> for Mask {
    fn from(s: S) -> Self {
        let inner = s
            .as_ref()
            .chars()
            .map(|c| match c {
                'X' => BitMask::None,
                '1' => BitMask::One,
                '0' => BitMask::Zero,
                _ => panic!("Invalid mask string"),
            })
            .collect();

        Self { inner }
    }
}

#[derive(Debug, Clone)]
struct Number {
    bits: [u8; 36],
}

impl From<usize> for Number {
    fn from(mut num: usize) -> Self {
        let mut bits = [0u8; 36];

        for bit in bits.iter_mut().rev() {
            if num < 2 {
                *bit = num as u8;
                break;
            } else {
                *bit = (num & 1) as u8;
                num >>= 1;
            }
        }

        Self { bits }
    }
}

impl From<Number> for usize {
    fn from(num: Number) -> Self {
        let mut dec = 0;
        for (idx, bit) in num.bits.iter().rev().enumerate() {
            if (bit & 1) == 1 {
                dec += 2usize.pow(idx as u32);
            }
        }

        dec
    }
}

impl Number {
    fn apply_mask(&mut self, mask: &Mask) {
        for (bit, change) in self.bits.iter_mut().zip(mask.inner.iter()) {
            match change {
                BitMask::None => continue,
                BitMask::One => *bit = 1,
                BitMask::Zero => *bit = 0,
            }
        }
    }

    fn addresses(&self, mask: &Mask) -> Vec<usize> {
        let mask = mask
            .inner
            .iter()
            .zip(self.bits.iter())
            .map(|(bitmask, bit)| match bitmask {
                BitMask::Zero => match bit {
                    0 => BitMask::Zero,
                    1 => BitMask::One,
                    _ => panic!("Invalid bit"),
                },
                other => *other,
            })
            .collect();

        let mask = Mask { inner: mask };

        let mut stack = vec![mask];
        let mut grab = 1;

        'main: loop {
            let mut generated = Vec::new();
            while generated.len() / 2 != grab {
                if let Some(mask) = stack.last() {
                    let floating = mask
                        .inner
                        .iter()
                        .enumerate()
                        .find_map(|(idx, bm)| matches!(bm, BitMask::None).as_some(idx));

                    if let Some(floating) = floating {
                        let mut tmp = mask.clone();
                        tmp.inner[floating] = BitMask::One;
                        generated.push(tmp.clone());
                        tmp.inner[floating] = BitMask::Zero;
                        generated.push(tmp);

                        stack.pop();
                    } else {
                        break 'main;
                    }
                } else {
                    break 'main;
                }
            }

            stack.append(&mut generated);
            grab *= 2;
        }

        stack
            .into_iter()
            .map(|mask| {
                mask.inner
                    .into_iter()
                    .enumerate()
                    .map(|(idx, bm)| match bm {
                        BitMask::Zero => 0,
                        BitMask::One => 2usize.pow(idx as u32),
                        BitMask::None => panic!("Invalid mask generated"),
                    })
                    .sum()
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
enum Inst {
    SetMask(String),
    SetMem(usize, usize),
}

#[derive(Debug, Clone)]
struct Program {
    insts: Vec<Inst>,
    mask: Option<Mask>,
    memory: HashMap<usize, usize>,
}

impl Program {
    fn load() -> AnyResult<Self> {
        fn parse(line: String) -> AnyResult<Inst> {
            let splitted: Vec<_> = line.split('=').map(|p| p.trim()).collect();

            let lhs = splitted.get(0).ok_or_else(|| anyhow!("Invalid line"))?;
            let rhs = splitted.get(1).ok_or_else(|| anyhow!("Invalid line"))?;

            Ok(match *lhs {
                "mask" => Inst::SetMask(rhs.to_string()),

                setmem => {
                    let addr = setmem
                        .get(4..setmem.len() - 1)
                        .ok_or_else(|| anyhow!("Invalid setmem"))?
                        .parse()?;

                    Inst::SetMem(addr, rhs.parse()?)
                }
            })
        }

        let insts: Vec<_> = lines("files/day14/input.txt")?
            .filter(|s| !s.is_empty())
            .map(parse)
            .collect::<AnyResult<_>>()?;

        Ok(Self {
            insts,
            mask: None,
            memory: HashMap::new(),
        })
    }

    fn reset(&mut self) {
        self.mask = None;
        self.memory.clear();
    }

    fn run_v1(&mut self) {
        for inst in &self.insts {
            match inst {
                Inst::SetMask(mask) => {
                    self.mask = Some(Mask::from(mask));
                }

                Inst::SetMem(addr, value) => {
                    if let Some(mask) = &self.mask {
                        let mut value = Number::from(*value);
                        value.apply_mask(mask);
                        self.memory.insert(*addr, usize::from(value));
                    } else {
                        panic!("Mask is not yet set");
                    }
                }
            }
        }
    }

    fn run_v2(&mut self) {
        for inst in &self.insts {
            match inst {
                Inst::SetMask(mask) => {
                    self.mask = Some(Mask::from(mask));
                }

                Inst::SetMem(addr, value) => {
                    if let Some(mask) = &self.mask {
                        let addresses = Number::from(*addr).addresses(mask);
                        for addr in addresses {
                            self.memory.insert(addr, *value);
                        }
                    } else {
                        panic!("Mask is not yet set")
                    }
                }
            }
        }
    }
}

fn main() -> AnyResult<()> {
    let mut program = Program::load()?;
    program.run_v1();
    let sum: usize = program.memory.values().sum();
    println!("Day 14, Part 1: {}", sum);

    program.reset();
    program.run_v2();
    let sum: usize = program.memory.values().sum();
    println!("Day 14, Part 2: {}", sum);

    Ok(())
}
