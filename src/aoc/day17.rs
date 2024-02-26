use grid::Grid;

pub struct Day17;

impl crate::aoc::Compute for Day17 {
    fn compute_part_one(&self, version: String) -> String {
        "TODO".to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        "TODO".to_string()
    }
}

impl Day17 {
    fn input_load(&self, part: String, version: String) -> City {
        City::new()
    }
}

struct City {
    blocks: Grid<char>,
    blocks_visited: Vec<Block>,
}

struct Block {
    x: isize,
    y: isize,
}