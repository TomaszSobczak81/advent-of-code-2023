use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

pub struct Day12;

impl crate::aoc::Compute for Day12 {
    fn compute_part_one(&self, version: String) -> String {
        // let springs = self.input_load("1".to_string(), version.clone());
        let mut permutations: usize = 0;
        // springs.iter().map(|s| s.count_possible_arrangements()).sum::<usize>().to_string()

        for s in self.input_load("1".to_string(), version.clone()) {
            permutations += s.count_possible_arrangements();
        }

        permutations.to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        "TODO".to_string()
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
    subsets_rgx: Regex,
    subsets_sum: usize,
}

impl Spring {
    fn new(pattern: String, subsets: Vec<usize>) -> Self {
        let subsets_rgx = Regex::new(&subsets.iter().map(|s| "#".repeat(*s)).collect::<Vec<String>>().join("[^#]+")).unwrap();
        let subsets_sum = subsets.iter().sum::<usize>();

        Self { pattern, subsets_sum, subsets_rgx }
    }

    fn count_possible_arrangements(&self) -> usize {
        self.possible_pattern_permutations().len()
    }

    fn possible_pattern_permutations(&self) -> HashSet<String> {
        let unknown_springs: usize = self.pattern.matches("?").count();
        let damaged_springs: usize = self.pattern.matches("#").count();

        let damaged_missing_springs: usize = self.subsets_sum - damaged_springs;
        let working_missing_springs: usize = unknown_springs - damaged_missing_springs;
        let unknown_missing_springs: Vec<String> = vec!["#"; damaged_missing_springs].into_iter().chain(vec!["."; working_missing_springs].into_iter()).map(|s| s.to_string()).collect();

        println!("pattern: {:?}, ?: {:?}, permutations: {:?}", self.pattern, unknown_springs, (1..=unknown_springs).product::<usize>());

        let mut permutations: HashSet<String> = HashSet::new();

        for p in unknown_missing_springs.iter().permutations(unknown_springs) {
            let p = self.fill_pattern_with_permutation(&p.into_iter().join(""));
            if self.validate_spring_against_pattern(&p) {
                permutations.insert(p);
            }
        }


        permutations
    }

    fn fill_pattern_with_permutation(&self, permutation: &String) -> String {
        let mut pattern = self.pattern.clone();

        for c in permutation.chars() {
            pattern = pattern.replacen("?", &c.to_string(), 1);
        }

        pattern
    }

    fn validate_spring_against_pattern(&self, pattern: &String) -> bool {
        if let Some(_) = self.subsets_rgx.find(&pattern) {
            return true;
        }

        false
    }
}