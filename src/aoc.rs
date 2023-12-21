use array2d::Array2D;
use clap::Args;
use grid::grid;
use grid::Grid;
use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;

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
    std::fs::read_to_string(format!("var/input/{version}/day{day}/part{part}.txt")).unwrap()
}

fn input_load_as_array2d(day: String, part: String, version: String) -> Array2D<char> {
    let input = crate::aoc::input_load(day, part, version);
    Array2D::from_rows(&input.lines().map(|l| l.chars().collect()).collect::<Vec<_>>()).unwrap()
}

fn input_load_as_grid(day: String, part: String, version: String) -> Grid<char> {
    let input = crate::aoc::input_load(day, part, version);
    let mut grid = grid![];

    for l in input.lines() {
        grid.push_row(l.chars().collect());
    }

    grid
}

fn math_gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { math_gcd(b, a % b) }
}

fn math_lcm(a: usize, b: usize) -> usize {
    (a * b) / math_gcd(a, b)
}

fn str_reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}
