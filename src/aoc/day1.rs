use clap::Args;
use regex::Regex;
use std::time::Instant;

#[derive(Args)]
pub struct Task {
    #[arg(default_value_t = String::from("day_1_trebuchet"))]
    name: String,
    #[arg(default_value_t = String::from("142"))]
    part_one_expected_test_value: String,
    #[arg(default_value_t = String::from("281"))]
    part_two_expected_test_value: String,
}

impl Task {
    fn check_solution(&self, part: i32, result: String, expected: String) {
        assert!(result == expected, "Test#{part} FAIL: Got {result} instead {expected}");
        println!("Test#{part} OK: Got {result} as expected");
    }

    fn solve_solution(&self, part: i32, expected_result: String) {
        match part {
            1 => self.check_solution(part, self.compute_part_one("test".to_string()), expected_result),
            2 => self.check_solution(part, self.compute_part_two("test".to_string()), expected_result),
            _ => panic!("Invalid part number"),
        }

        let start = Instant::now();
        let result = match part {
            1 => self.compute_part_one("live".to_string()),
            2 => self.compute_part_two("live".to_string()),
            _ => panic!("Invalid part number"),
        };
        let duration = start.elapsed();

        println!("Part#{part} result: {result} computed in {duration:?}");
    }

    fn compute_part_one(&self, version: String) -> String {
        let input = self.load_input("1".to_string(), version.clone());

        return self.compute(input);
    }

    fn compute_part_two(&self, version: String) -> String {
        let s = self.load_input("2".to_string(), version.clone());
        let input: Vec<_> = s.lines().map(|line| self.normalize_input_line(line)).collect();

        return self.compute(input.join("\n"));
    }

    fn normalize_input_line(&self, line: &str) -> String {
        let mut result = line.to_string();

        //TODO: Analuzye and normalize input line

        return result;
    }

    fn compute(&self, input: String) -> String {
        let mut result: i32 = 0;

        for line in input.lines() {
            let enil: String = line.chars().rev().collect();
            let re = Regex::new(r"(\d)").unwrap();
            let f = re.find(line).unwrap().as_str();
            let l = re.find(enil.as_str()).unwrap().as_str();

            result += format!("{}{}", f, l).parse::<i32>().unwrap();
        }

        return result.to_string();
    }

    fn load_input(&self, part: String, version: String) -> String {
        let filename = format!("var/input/{name}_part_{part}_{version}.txt", name = self.name);
        let contents = std::fs::read_to_string(filename)
        .unwrap();

        return contents;
    }

    pub fn solve(&self) {
        self.solve_solution(1, self.part_one_expected_test_value.to_string());
        self.solve_solution(2, self.part_two_expected_test_value.to_string());
    }
}
