use regex::Regex;

pub struct Day12;

impl crate::aoc::Compute for Day12 {
    fn compute_part_one(&self, version: String) -> String {
        let springs = self.input_load("1".to_string(), version.clone());
        springs.iter().map(|s| s.count_possible_arrangements()).sum::<usize>().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let springs = self.input_load("1".to_string(), version.clone());
        springs.iter().map(|s| s.count_possible_arrangements()).sum::<usize>().to_string()
    }
}

impl Day12 {
    fn input_load(&self, part: String, version: String) -> Vec<Spring> {
        let mut springs: Vec<Spring> = vec![];

        for line in crate::aoc::input_load("12".to_string(), part, version).lines() {
            let mut parts = line.split(" ");
            let pattern: String = parts.next().unwrap().to_string();
            let subsets: Vec<usize> = parts.next().unwrap().split(",").map(|s| s.trim().parse::<usize>().unwrap()).collect();

            springs.push(Spring::new(pattern, subsets));
        }

        springs
    }
}

struct Spring {
    pattern: String,
    pattern_rgx: Regex,
}

impl Spring {
    fn new(pattern: String, subsets: Vec<usize>) -> Self {
        let springs_rgx = subsets.iter().map(|s| format!("[#]{{{}}}", *s)).collect::<Vec<String>>().join("[^#]+");
        let pattern_rgx = Regex::new(&format!(r"^(\.*){}(\.*)$", springs_rgx)).unwrap();

        Self { pattern, pattern_rgx }
    }

    fn count_possible_arrangements(&self) -> usize {
        self.process_arrangement(self.pattern.clone(), 0)
    }

    fn process_arrangement(&self, arrangement: String, sum: usize) -> usize {
        let mut local_sum = sum;

        if arrangement.contains("?") {
            for c in ['.', '#'].iter() {
                local_sum = self.process_arrangement(arrangement.replacen("?", &c.to_string(), 1), local_sum);
            }

            return local_sum;
        }

        match self.validate_spring_against_pattern(&arrangement) {
            true => local_sum + 1,
            false => local_sum,
        }
    }

    fn validate_spring_against_pattern(&self, pattern: &String) -> bool {
        match self.pattern_rgx.find(&pattern) {
            Some(_) => true,
            None => false,
        }
    }
}