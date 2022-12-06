use itertools::Itertools;

// PART 1

fn sum_lines(lines: Vec<&str>) -> u32 {
    return lines
        .iter()
        .map(|l| str::parse::<u32>(l).unwrap_or(0))
        .sum();
}

fn sorted_sums(input: &str) -> Vec<u32> {
    let mut groups: Vec<Vec<&str>> = Vec::new();
    for (key, group) in &input.lines().group_by(|l| !l.is_empty()) {
        if key {
            groups.push(group.collect());
        }
    }

    let mut sums: Vec<u32> = groups
        .iter()
        .map(|lines| sum_lines(lines.to_vec()))
        .collect();

    sums.sort();
    sums
}

pub fn solve_1(input: &str) -> String {
    let sums = sorted_sums(input);
    return sums.last().unwrap_or(&0).to_string();
}

// PART 2

pub fn solve_2(input: &str) -> String {
    let sums = sorted_sums(input);
    let total: u32 = sums.iter().rev().take(3).sum();

    total.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn solve_1_correct() {
        let result = solve_1(TEST_INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn solve_2_correct() {
        let result = solve_2(TEST_INPUT);
        assert_eq!(result, "45000");
    }
}
