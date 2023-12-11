use crate::aoc::math_lcm;
use regex::Regex;
use std::collections::HashMap;

pub struct Day8;

impl crate::aoc::Compute for Day8 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        let mut steps: usize = 0;
        let mut cnode = input.nodes.get("AAA").unwrap();

        while !cnode.name.eq("ZZZ") {
            cnode = input.next_node(steps, cnode);
            steps += 1;
        }

        steps.to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load("2".to_string(), version.clone());
        let mut steps: Vec<usize> = Vec::new();
        let mut nodes = input.nodes_ending_with_a();

        for cnode in nodes.iter_mut() {
            let mut csteps: usize = 0;

            while !cnode.name.ends_with("Z") {
                *cnode = input.next_node(csteps, cnode);
                csteps += 1;
            }

            steps.push(csteps);
        }

        steps.iter().map(|s| *s ).reduce(|a, b| math_lcm(a, b)).unwrap().to_string()
    }
}

impl Day8 {
    fn input_load(&self, part: String, version: String) -> DessertMap {
        let input = crate::aoc::input_load("8".to_string(), part.clone(), version.clone()); 
        let steps_regex = Regex::new(r"^[LR]+$").unwrap();
        let nodes_regex = Regex::new(r"(?P<name>[A-Z\d]+)\s+=\s+\((?P<left>[A-Z\d]+),\s+(?P<right>[A-Z\d]+)\)").unwrap();

        let mut steps: Vec<char> = Vec::new();
        let mut nodes: HashMap<String, DessertNode> = HashMap::new();

        for line in input.lines() {
            if steps_regex.is_match(line) {
                steps = line.chars().collect();
            }

            if let Some(captures) = nodes_regex.captures(line) {
                nodes.insert(
                    captures.name("name").unwrap().as_str().to_string(),
                    DessertNode {
                        name: captures.name("name").unwrap().as_str().to_string(),
                        left: captures.name("left").unwrap().as_str().to_string(),
                        right: captures.name("right").unwrap().as_str().to_string(),
                    }
                );
            }
        }

        DessertMap {
            steps: steps,
            nodes: nodes,
        }
    }
}

#[derive(Debug)]
struct DessertNode {
    name: String,
    left: String,
    right: String,
}

struct DessertMap {
    steps: Vec<char>,
    nodes: HashMap<String, DessertNode>,
}

impl DessertMap {
    fn next_node(&self, index: usize, cnode: &DessertNode) -> &DessertNode {
        match self.steps[index % self.steps.len()] {
            'L' => self.nodes.get(&cnode.left).unwrap(),
            'R' => self.nodes.get(&cnode.right).unwrap(),
            _ => panic!("Invalid step"),
        }
    }

    fn nodes_ending_with_a(&self) -> Vec<&DessertNode> {
        self.nodes.values().filter(|&node| node.name.ends_with("A")).collect()
    }
}
