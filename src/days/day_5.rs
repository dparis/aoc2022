// PART 1

pub fn solve_1(input: &str) -> String {
    return "NA".to_string();
}

// PART 2

pub fn solve_2(input: &str) -> String {
    return "NA".to_string();
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

    // #[test]
    // fn solve_2_correct() {
    //     let result = solve_2(TEST_INPUT);
    //     assert_eq!(result, "4");
    // }
}
