use itertools::Itertools;
use std::collections::HashMap;

// PART 1

struct Sack<'a> {
    contents: &'a str,
    compartment_1: Option<&'a str>,
    compartment_2: Option<&'a str>,
}

fn split_line(line: &str) -> Option<(&str, &str)> {
    let len = line.len();

    if len == 0 {
        None
    } else {
        let half_idx = len / 2;
        Some((&line[0..half_idx], &line[half_idx..]))
    }
}

impl<'a> Sack<'a> {
    fn new(contents: &str) -> Sack {
        let compartments = split_line(contents);

        Sack {
            contents,
            compartment_1: compartments.map(|c| c.0),
            compartment_2: compartments.map(|c| c.1),
        }
    }
}

fn parse_input(input: &str) -> Vec<Sack> {
    return input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Sack::new)
        .collect();
}

fn first_common_char(strings: Vec<&str>) -> Option<char> {
    let string_count = strings.len();
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for s in strings.iter() {
        for c in s.chars().unique() {
            let mut count = *char_counts.get(&c).unwrap_or(&0);
            count += 1;

            if count == string_count {
                return Some(c);
            } else {
                char_counts.insert(c, count);
            }
        }
    }

    None
}

fn find_duplicate_item(sack: &Sack) -> Option<char> {
    let strings = vec![sack.compartment_1?, sack.compartment_2?];
    first_common_char(strings)
}

fn priority(item: char) -> u32 {
    let ascii_val = item as u32;

    if item.is_lowercase() {
        ascii_val - 96
    } else {
        ascii_val - 38
    }
}

pub fn solve_1(input: &str) -> String {
    let sacks = parse_input(input);

    let dupe_priority_sum: u32 = sacks
        .iter()
        .filter_map(find_duplicate_item)
        .map(priority)
        .sum();

    dupe_priority_sum.to_string()
}

// PART 2

fn find_badge(group: &[Sack]) -> Option<char> {
    let strings = group.iter().map(|sack| sack.contents).collect();
    first_common_char(strings)
}

pub fn solve_2(input: &str) -> String {
    let sacks = parse_input(input);

    let sack_groups: Vec<Vec<Sack>> = sacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|group| group.collect())
        .collect();

    let badge_priority_sum: u32 = sack_groups
        .iter()
        .filter_map(|group| find_badge(group))
        .map(priority)
        .sum();

    badge_priority_sum.to_string()
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

    #[test]
    fn solve_2_correct() {
        let result = solve_2(TEST_INPUT);
        assert_eq!(result, "70");
    }
}
