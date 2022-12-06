// PART 1

use itertools::Itertools;

struct Assignment {
    min: u32,
    max: u32,
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.max >= other.min && self.max <= other.max
            || self.min >= other.min && self.min <= other.max
            || self.min <= other.min && self.max >= other.max
    }
}

type AssignmentPair = (Assignment, Assignment);

fn parse_input_line(line: &str) -> Option<AssignmentPair> {
    let (left, right) = line.split(',').collect_tuple()?;

    let (left_min, left_max) = left
        .split('-')
        .filter_map(|x| x.parse().ok())
        .collect_tuple()?;

    let (right_min, right_max) = right
        .split('-')
        .filter_map(|x| x.parse().ok())
        .collect_tuple()?;

    let la = Assignment {
        min: left_min,
        max: left_max,
    };
    let ra = Assignment {
        min: right_min,
        max: right_max,
    };

    Some((la, ra))
}

fn parse_input(input: &str) -> Vec<AssignmentPair> {
    return input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(parse_input_line)
        .collect();
}

fn containing_pair(pair: &AssignmentPair) -> bool {
    pair.0.contains(&pair.1) || pair.1.contains(&pair.0)
}

fn overlappying_pair(pair: &AssignmentPair) -> bool {
    pair.0.overlaps(&pair.1)
}

pub fn solve_1(input: &str) -> String {
    let pairs = parse_input(input);

    let containing_pairs = pairs.iter().filter(|&pair| containing_pair(pair)).count();

    containing_pairs.to_string()
}

// PART 2

pub fn solve_2(input: &str) -> String {
    let pairs = parse_input(input);

    let overlapping_pairs = pairs.iter().filter(|&pair| overlappying_pair(pair)).count();

    overlapping_pairs.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn solve_1_correct() {
        let result = solve_1(TEST_INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn solve_2_correct() {
        let result = solve_2(TEST_INPUT);
        assert_eq!(result, "4");
    }
}
