// PART 1

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Crate {
    label: String,
}

type StackId = u32;

#[derive(Debug)]
struct SupplyStack {
    id: StackId,
    stack: Vec<Crate>,
}

#[derive(Clone, Debug)]
struct Instruction {
    amount: u32,
    src: StackId,
    dest: StackId,
}

#[derive(Debug)]
struct CargoManifest {
    stacks: HashMap<StackId, SupplyStack>,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
enum InstructionError {
    SrcNotFound(Instruction),
    DestNotFound(Instruction),
    InvalidAmount(Instruction),
}

type InstructionResult = std::result::Result<(), InstructionError>;

impl CargoManifest {
    fn apply_instructions_part_1(&mut self) -> InstructionResult {
        for inst in self.instructions.iter() {
            let amount = inst.amount;

            let mut src = self
                .stacks
                .remove(&inst.src)
                .ok_or_else(|| InstructionError::SrcNotFound(inst.clone()))?;

            let mut dest = self
                .stacks
                .remove(&inst.dest)
                .ok_or_else(|| InstructionError::DestNotFound(inst.clone()))?;

            for _ in 0..amount {
                let c = src
                    .stack
                    .pop()
                    .ok_or_else(|| InstructionError::InvalidAmount(inst.clone()))?;
                dest.stack.push(c);
            }

            self.stacks.insert(src.id, src);
            self.stacks.insert(dest.id, dest);
        }

        Ok(())
    }

    fn apply_instructions_part_2(&mut self) -> InstructionResult {
        for inst in self.instructions.iter() {
            let amount = inst.amount;
            let mut src = self
                .stacks
                .remove(&inst.src)
                .ok_or_else(|| InstructionError::SrcNotFound(inst.clone()))?;
            let mut dest = self
                .stacks
                .remove(&inst.dest)
                .ok_or_else(|| InstructionError::DestNotFound(inst.clone()))?;

            let split_at = src
                .stack
                .len()
                .checked_sub(amount as usize)
                .ok_or_else(|| InstructionError::InvalidAmount(inst.clone()))?;

            let mut load = src.stack.split_off(split_at);
            dest.stack.append(&mut load);

            self.stacks.insert(src.id, src);
            self.stacks.insert(dest.id, dest);
        }

        Ok(())
    }

    fn current_tops(&self) -> Vec<Option<&Crate>> {
        return self
            .stacks
            .values()
            .sorted_by_key(|s| s.id)
            .map(|s| s.stack.last())
            .collect();
    }
}

fn parse_stacks(lines: &[&str]) -> Option<HashMap<StackId, SupplyStack>> {
    let stack_ids = lines.last()?;
    let id_indexes: Vec<usize> = stack_ids
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c.is_whitespace() { None } else { Some(i) })
        .collect();

    let mut stacks: HashMap<StackId, SupplyStack> = HashMap::new();

    for (id, idx) in id_indexes.iter().enumerate() {
        let stack_id = id as u32 + 1;

        let stack: Vec<Crate> = lines
            .iter()
            .rev()
            .skip(1)
            .filter_map(|line| match line.chars().nth(*idx) {
                Some(label) if label.is_alphanumeric() => Some(Crate {
                    label: label.to_string(),
                }),
                Some(_) => None,
                None => None,
            })
            .collect();

        let supply_stack = SupplyStack {
            id: stack_id,
            stack,
        };

        stacks.insert(stack_id, supply_stack);
    }

    Some(stacks)
}

fn parse_instructions(lines: Vec<&str>) -> Option<Vec<Instruction>> {
    let r = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("Invalid regex");

    let instructions = lines
        .iter()
        .filter_map(|line| r.captures(line))
        .filter_map(|capture| {
            let amount = capture.get(1)?.as_str().parse().ok()?;
            let src = capture.get(2)?.as_str().parse().ok()?;
            let dest = capture.get(3)?.as_str().parse().ok()?;

            Some(Instruction { amount, src, dest })
        })
        .collect();

    Some(instructions)
}

fn parse_input(input: &str) -> Option<CargoManifest> {
    let stack_diagram_lines: Vec<&str> = input
        .lines()
        .skip_while(|l| l.is_empty())
        .take_while(|l| !l.is_empty())
        .collect();

    let stacks = parse_stacks(&stack_diagram_lines)?;

    let instruction_lines: Vec<&str> = input
        .lines()
        .skip_while(|l| l.is_empty())
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .collect();

    let instructions = parse_instructions(instruction_lines)?;

    let cm = CargoManifest {
        stacks,
        instructions,
    };

    Some(cm)
}

pub fn solve_1(input: &str) -> String {
    let mut cargo_manifest = parse_input(input).expect("Invalid input");

    if let Err(e) = cargo_manifest.apply_instructions_part_1() {
        eprintln!("error applying instructions: {:?} {:?}", e, cargo_manifest);
        return String::from("ERROR");
    }

    let tops: Vec<&String> = cargo_manifest
        .current_tops()
        .iter()
        .filter_map(|crate_opt| crate_opt.as_ref().map(|c| &c.label))
        .collect();

    return tops.iter().join("");
}

// PART 2

pub fn solve_2(input: &str) -> String {
    let mut cargo_manifest = parse_input(input).expect("Invalid input");

    if let Err(e) = cargo_manifest.apply_instructions_part_2() {
        eprintln!("error applying instructions: {:?} {:?}", e, cargo_manifest);
        return String::from("ERROR");
    }

    let tops: Vec<&String> = cargo_manifest
        .current_tops()
        .iter()
        .filter_map(|crate_opt| crate_opt.as_ref().map(|c| &c.label))
        .collect();

    return tops.iter().join("");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn solve_1_correct() {
        let result = solve_1(TEST_INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn solve_2_correct() {
        let result = solve_2(TEST_INPUT);
        assert_eq!(result, "MCD");
    }
}
