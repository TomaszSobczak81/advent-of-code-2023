use clap::Args;
use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;

pub trait Compute {
    fn compute_part_one(&self, version: String) -> String;
    fn compute_part_two(&self, version: String) -> String;
}

#[derive(Args)]
pub struct Task {
}

impl Task {
    fn compute_result(&self, part: i32, compute: &impl Compute, version: String) -> String {
        match part {
            1 => compute.compute_part_one(version),
            2 => compute.compute_part_two(version),
            _ => panic!("Invalid part number"),
        }
    }

    pub fn solve(&self, compute: &impl Compute, part_one_expected_test_value: String, part_two_expected_test_value: String) {
        self.solve_solution(1, compute, part_one_expected_test_value);
        self.solve_solution(2, compute, part_two_expected_test_value);
    }

    fn solve_solution(&self, part: i32, compute: &impl Compute, expected_result: String) {
        self.test_solution(part, compute, expected_result);

        let start = Instant::now();
        let result = self.compute_result(part, compute, "live".to_string());
        let duration = start.elapsed();

        println!("Part#{part} result: {result} computed in {duration:?}");
    }

    fn test_solution(&self, part: i32, compute: &impl Compute, expected_result: String) {
        let result = self.compute_result(part, compute, "test".to_string());

        assert!(result == expected_result, "Test#{part} FAIL: Got {result} instead {expected_result}");
        println!("Test#{part} OK: Got {result} as expected");
    }
}

fn input_load(day: String, part: String, version: String) -> String {
    std::fs::read_to_string(format!("var/input/day{day}_part{part}_{version}.txt")).unwrap()
}

fn str_reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}
