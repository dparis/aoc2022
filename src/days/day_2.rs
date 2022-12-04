use itertools::Itertools;

// PART 1

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    return input
        .lines()
        .filter_map(|line| line.split(" ").collect_tuple())
        .collect()
}

#[derive(Debug)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

fn challenger_throw(code: &str) -> Option<Throw> {
    return match code {
        "A" => Some(Throw::Rock),
        "B" => Some(Throw::Paper),
        "C" => Some(Throw::Scissors),
        _ => None,
    };
}

fn player_throw(code: &str) -> Option<Throw> {
    return match code {
        "X" => Some(Throw::Rock),
        "Y" => Some(Throw::Paper),
        "Z" => Some(Throw::Scissors),
        _ => None,
    };
}

#[derive(Debug)]
enum Outcome {
    ChallengerWins(Throw),
    PlayerWins(Throw),
    Draw(Throw),
}

fn play_round(challenger_code: &str, player_code: &str) -> Option<Outcome> {
    let ct = challenger_throw(challenger_code)?;
    let pt = player_throw(player_code)?;

    let result = match (ct, pt) {
        (Throw::Rock, Throw::Scissors) => Outcome::ChallengerWins(Throw::Scissors),
        (Throw::Paper, Throw::Rock) => Outcome::ChallengerWins(Throw::Rock),
        (Throw::Scissors, Throw::Paper) => Outcome::ChallengerWins(Throw::Paper),

        (Throw::Scissors, Throw::Rock) => Outcome::PlayerWins(Throw::Rock),
        (Throw::Rock, Throw::Paper) => Outcome::PlayerWins(Throw::Paper),
        (Throw::Paper, Throw::Scissors) => Outcome::PlayerWins(Throw::Scissors),

        (Throw::Rock, Throw::Rock) => Outcome::Draw(Throw::Rock),
        (Throw::Paper, Throw::Paper) => Outcome::Draw(Throw::Paper),
        (Throw::Scissors, Throw::Scissors) => Outcome::Draw(Throw::Scissors),
    };

    return Some(result);
}

fn throw_score(throw: Throw) -> u32 {
    return match throw {
        Throw::Rock => 1,
        Throw::Paper => 2,
        Throw::Scissors => 3,
    };
}

fn round_score(outcome: Outcome) -> u32 {
    return match outcome {
        Outcome::PlayerWins(throw) => 6 + throw_score(throw),
        Outcome::Draw(throw) => 3 + throw_score(throw),
        Outcome::ChallengerWins(throw) => throw_score(throw),
    };
}

pub fn solve_1(input: &str) -> String {
    let plays = parse_input(input);

    let score: u32 = plays
        .iter()
        .filter_map(|(cc, pc)| play_round(cc, pc))
        .map(|outcome| round_score(outcome))
        .sum();

    return score.to_string();
}

// PART 2

enum OutcomeInstruction {
    Win,
    Lose,
    Draw
}

fn outcome_instruction(outcome_code: &str) -> Option<OutcomeInstruction> {
    return match outcome_code {
        "X" => Some(OutcomeInstruction::Lose),
        "Y" => Some(OutcomeInstruction::Draw),
        "Z" => Some(OutcomeInstruction::Win),
        _ => None
    }
}

fn desired_outcome(challenger_code: &str, outcome_code: &str) -> Option<Outcome> {
    let ct = challenger_throw(challenger_code)?;
    let oi = outcome_instruction(outcome_code)?;

    let result = match (ct, oi) {
        (Throw::Rock, OutcomeInstruction::Win) => Outcome::PlayerWins(Throw::Paper),
        (Throw::Rock, OutcomeInstruction::Lose) => Outcome::ChallengerWins(Throw::Scissors),
        (Throw::Rock, OutcomeInstruction::Draw) => Outcome::Draw(Throw::Rock),

        (Throw::Paper, OutcomeInstruction::Win) => Outcome::PlayerWins(Throw::Scissors),
        (Throw::Paper, OutcomeInstruction::Lose) => Outcome::ChallengerWins(Throw::Rock),
        (Throw::Paper, OutcomeInstruction::Draw) => Outcome::Draw(Throw::Paper),

        (Throw::Scissors, OutcomeInstruction::Win) => Outcome::PlayerWins(Throw::Rock),
        (Throw::Scissors, OutcomeInstruction::Lose) => Outcome::ChallengerWins(Throw::Paper),
        (Throw::Scissors, OutcomeInstruction::Draw) => Outcome::Draw(Throw::Scissors),
    };

    return Some(result);
}

pub fn solve_2(input: &str) -> String {
    let plays = parse_input(input);

    let score: u32 = plays
        .iter()
        .filter_map(|(cc, oc)| desired_outcome(cc, oc))
        .map(|outcome| round_score(outcome))
        .sum();

    return score.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"
A Y
B X
C Z
"#;

    #[test]
    fn solve_1_correct() {
        let result = solve_1(TEST_INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn solve_2_correct() {
        let result = solve_2(TEST_INPUT);
        assert_eq!(result, "12");
    }
}
