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
        let input = self.load_input("2".to_string(), version.to_string());
        let lines: Vec<_> = input.lines().map(|line| self.normalize_input_line(line)).collect();

        return self.compute(lines.join("\n"));
    }

    fn normalize_input_line(&self, line: &str) -> String {
        let forward_mapping = std::collections::HashMap::from([
            ("zero", "0"),
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9")
        ]);

        let reverse_mapping = std::collections::HashMap::from([
            ("orez", "0"),
            ("eno", "1"),
            ("owt", "2"),
            ("eerht", "3"),
            ("ruof", "4"),
            ("evif", "5"),
            ("xis", "6"),
            ("neves", "7"),
            ("thgie", "8"),
            ("enin", "9")
        ]);

        let forward_pattern = forward_mapping.keys().map(|s| s.to_string()).collect::<Vec<_>>().join("|");
        let reverse_pattern = reverse_mapping.keys().map(|s| s.to_string()).collect::<Vec<_>>().join("|");

        let numeral_regex = Regex::new(r"(\d)").unwrap();
        let forward_regex = Regex::new(&forward_pattern).unwrap();
        let reverse_regex = Regex::new(&reverse_pattern).unwrap();

        let mut result = line.to_string();
        let mut finding = forward_regex.find(&result);

        if let Some(f) = finding {
            let number = numeral_regex.find(&result);
            if number.is_none() || number.unwrap().start() > f.start() {
                result = forward_regex.replace(&result, &**forward_mapping.get(f.as_str()).unwrap()).to_string();
            }
        }

        result = crate::aoc::str_reverse(&result); // reverse the line to find on the other side
        finding = reverse_regex.find(&result);

        if let Some(f) = finding {
            let number = numeral_regex.find(&result);
            if number.is_none() || number.unwrap().start() > f.start() {
                result = reverse_regex.replace(&result, &**reverse_mapping.get(f.as_str()).unwrap()).to_string();
            }
        }

        result = crate::aoc::str_reverse(&result); // back to initial order
        result
    }

    fn compute(&self, input: String) -> String {
        let mut result: i32 = 0;
        let re = Regex::new(r"(\d)").unwrap();

        for line in input.lines() {
            let enil = crate::aoc::str_reverse(line);
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

        contents
    }

    pub fn solve(&self) {
        self.solve_solution(1, self.part_one_expected_test_value.to_string());
        self.solve_solution(2, self.part_two_expected_test_value.to_string());
    }
}
