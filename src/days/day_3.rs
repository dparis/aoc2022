use std::collections::HashSet;

// PART 1

fn split_line(line: &str) -> Option<(&str, &str)> {
    let len = line.len();

    if len == 0 {
        return None;
    } else {
        let half_idx = len / 2;
        return Some((&line[0..half_idx], &line[half_idx..]));
    }
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    return input.lines().filter_map(split_line).collect();
}

fn find_duplicate(sack: (&str, &str)) -> Option<char> {
    let compartment_1: HashSet<char> = HashSet::from_iter(sack.0.chars());
    let compartment_2: HashSet<char> = HashSet::from_iter(sack.1.chars());

    let dupe = compartment_1
        .intersection(&compartment_2)
        .next()?;

    return Some(*dupe);
}

fn priority(item: char) -> u32 {
    let ascii_val = item as u32;

    if item.is_lowercase() {
        return ascii_val - 96;
    } else {
        return ascii_val - 38;
    }
}

pub fn solve_1(input: &str) -> String {
    let sacks = parse_input(input);

    let dupe_priority_sum: u32 = sacks
        .iter()
        .filter_map(|sack| find_duplicate(*sack))
        .map(|dupe| priority(dupe))
        .sum();

    return dupe_priority_sum.to_string();
}

// PART 2

pub fn solve_2(input: &str) -> String {
    return "NA".to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn solve_1_correct() {
        let result = solve_1(TEST_INPUT);
        assert_eq!(result, "157");
    }

    // #[test]
    // fn solve_2_correct() {
    //     let result = solve_2(TEST_INPUT);
    //     assert_eq!(result, "12");
    // }
}
