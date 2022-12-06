// PART 1

use std::collections::HashSet;

fn parse_input(input: &str) -> String {
    return input.trim().to_string();
}

fn find_marker_index(input: &str, marker_len: usize) -> Option<usize> {
    let mut found: HashSet<char> = HashSet::new();

    for index in 0..(input.len() - marker_len) {
        let s = &input[index..(index + marker_len)];

        for c in s.chars() {
            found.insert(c);
        }

        if found.len() == marker_len {
            return Some(index + marker_len);
        }

        found.clear();
    }

    None
}

pub fn solve_1(input: &str) -> String {
    let data = parse_input(input);
    find_marker_index(&data, 4).map_or(String::from(""), |i| i.to_string())
}

// PART 2

pub fn solve_2(input: &str) -> String {
    let data = parse_input(input);
    find_marker_index(&data, 14).map_or(String::from(""), |i| i.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
mjqjpqmgbljsphdztnvjfqwrcgsmlb
"#;

    #[test]
    fn solve_1_correct() {
        let result = solve_1(TEST_INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn solve_2_correct() {
        let result = solve_2(TEST_INPUT);
        assert_eq!(result, "19");
    }
}
