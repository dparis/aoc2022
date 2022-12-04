mod day_1;
mod day_2;
mod day_3;
mod day_4;

use once_cell::unsync::OnceCell;
use std::fs;

type SolverFn = fn(&str) -> String;

pub struct Problem {
    path: String,
    solver: Option<SolverFn>,
    solution: OnceCell<String>,
}

impl Problem {
    fn read_input(&self) -> Option<String> {
        return fs::read_to_string(&self.path).ok();
    }

    pub fn solve(&self) -> Option<&str> {
        let solver = self.solver?;
        let input = self.read_input()?;

        let solution = self.solution.get_or_init(|| solver(&input));
        return Some(solution);
    }
}

pub enum Correct {
    None,
    PartOne,
    Both,
}

pub struct Day {
    advent_day: u8,
    title: String,
    correct: Correct,
    pub part_1: Problem,
    pub part_2: Problem,
}

fn input_path(advent_day: u8, input_part: u8) -> String {
    return format!("./inputs/day_{}/input_{}.txt", advent_day, input_part);
}

impl Day {
    pub fn new(
        advent_day: u8,
        title: String,
        first_solver: Option<SolverFn>,
        second_solver: Option<SolverFn>,
        correct: Correct,
    ) -> Self {
        Self {
            advent_day,
            title,
            part_1: Problem {
                path: input_path(advent_day, 1),
                solver: first_solver,
                solution: OnceCell::new(),
            },
            part_2: Problem {
                path: input_path(advent_day, 2),
                solver: second_solver,
                solution: OnceCell::new(),
            },
            correct,
        }
    }

    pub fn label(&self) -> String {
        return format!("Day {} - {}", self.advent_day, self.title);
    }

    pub fn url(&self) -> String {
        return format!("https://adventofcode.com/2022/day/{}", self.advent_day);
    }

    pub fn stars(&self) -> String {
        return match self.correct {
            Correct::None => String::from(""),
            Correct::PartOne => String::from("*"),
            Correct::Both => String::from("**"),
        };
    }
}

pub fn init_days() -> Vec<Day> {
    return vec![
        Day::new(
            1,
            String::from("Calorie Counting"),
            Some(day_1::solve_1),
            Some(day_1::solve_2),
            Correct::Both,
        ),
        Day::new(
            2,
            String::from("Rock Paper Scissors"),
            Some(day_2::solve_1),
            Some(day_2::solve_2),
            Correct::Both,
        ),
        Day::new(
            3,
            String::from("Rucksack Reorganization"),
            Some(day_3::solve_1),
            Some(day_3::solve_2),
            Correct::PartOne,
        ),
        Day::new(
            4,
            String::from("Camp Cleanup"),
            Some(day_4::solve_1),
            Some(day_4::solve_2),
            Correct::None,
        ),
    ];
}
