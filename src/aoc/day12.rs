use regex::Regex;

pub struct Day12;

impl crate::aoc::Compute for Day12 {
    fn compute_part_one(&self, version: String) -> String {
        let springs = self.input_load("1".to_string(), version.clone(), false);
        springs.iter().map(|s| s.count_possible_arrangements()).sum::<usize>().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let springs = self.input_load("1".to_string(), version.clone(), true);
        springs.iter().map(|s| s.count_possible_arrangements()).sum::<usize>().to_string()
    }
}

impl Day12 {
    fn input_load(&self, part: String, version: String, unfold: bool) -> Vec<Spring> {
        let mut springs: Vec<Spring> = vec![];

        for line in crate::aoc::input_load("12".to_string(), part, version).lines() {
            let mut parts = line.split(" ");
            let pattern: String = parts.next().unwrap().to_string();
            let subsets: Vec<usize> = parts.next().unwrap().split(",").map(|s| s.trim().parse::<usize>().unwrap()).collect();

            match unfold {
                false => springs.push(Spring::new(pattern, subsets)),
                true => {
                    let mut unfolded_pattern = vec![pattern.clone(); 5].join("?");
                    let mut unfolded_subsets = Vec::new();

                    if pattern.starts_with("#") || pattern.ends_with("#") {
                        unfolded_pattern = vec![pattern.clone(); 5].join(".");
                    }

                    for _ in 1..=5 {
                        unfolded_subsets.extend(subsets.clone());
                    }

                    springs.push(Spring::new(unfolded_pattern, unfolded_subsets));
                }
            }
        }

        springs
    }
}

struct Spring {
    pattern: String,
    pattern_rgx: Regex,
    damaged_len: usize,
}

impl Spring {
    fn new(mut pattern: String, subsets: Vec<usize>) -> Self {
        let springs_rgx = subsets.iter().map(|s| format!("[#]{{{}}}", *s)).collect::<Vec<String>>().join("[^#]+");
        let pattern_rgx = Regex::new(&format!(r"^(\.*){}(\.*)$", springs_rgx)).unwrap();
        let damaged_len = subsets.iter().sum::<usize>();

        if pattern.starts_with("#") {
            pattern.replace_range(..=subsets[0], format!("{}{}", &"#".repeat(subsets[0]), ".").as_str());
        }

        if pattern.ends_with("#") {
            pattern.replace_range(pattern.len() - subsets.last().unwrap() - 1.., format!("{}{}", ".", &"#".repeat(*subsets.last().unwrap())).as_str());
        }

        Self { pattern, pattern_rgx, damaged_len }
    }

    fn count_possible_arrangements(&self) -> usize {
        self.process_arrangement(self.pattern.clone(), 0)
    }

    fn process_arrangement(&self, arrangement: String, sum: usize) -> usize {
        let mut local_sum = sum;

        if arrangement.contains("?") {
            let damaged_count = arrangement.matches("#").count();
            let unknown_count = arrangement.matches("?").count();

            if damaged_count >= self.damaged_len {
                local_sum = self.process_arrangement(arrangement.replace("?", "."), local_sum);
            } else if damaged_count + unknown_count == self.damaged_len {
                local_sum = self.process_arrangement(arrangement.replace("?", "#"), local_sum);
            } else {
                local_sum = self.process_arrangement(arrangement.replacen("?", "#", 1), local_sum);
                local_sum = self.process_arrangement(arrangement.replacen("?", ".", 1), local_sum);
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