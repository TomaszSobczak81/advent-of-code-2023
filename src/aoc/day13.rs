use grid::Grid;
use grid::grid;
use std::cmp;
use std::collections::HashMap;

pub struct Day13;

impl crate::aoc::Compute for Day13 {
    fn compute_part_one(&self, version: String) -> String {
        let patterns = self.input_load("1".to_string(), version.clone());
        patterns.iter().map(|p| p.reflection_score()).sum::<usize>().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let patterns = self.input_load("1".to_string(), version.clone());
        patterns.iter().map(|p| p.reflection_score_without_smudge()).sum::<usize>().to_string()
    }
}

impl Day13 {
    fn input_load(&self, part: String, version: String) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = vec![];
        let mut grid = grid![];

        for line in crate::aoc::input_load("13".to_string(), part, version).lines() {
            if line.is_empty() {
                patterns.push(Pattern::new(grid));

                grid = grid![];
                continue;
            }

            grid.push_row(line.chars().collect());
        }

        if !grid.is_empty() {
            patterns.push(Pattern::new(grid));
        }

        patterns
    }
}

struct Pattern {
    grid: Grid<char>
}

impl Pattern {
    fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    fn find_reflection_score(&self, grid: &Grid<char>, refl_min: usize) -> usize {
        let mut prev: Vec<String> = vec![];
        let mut next: Vec<String> = vec![];
        let mut refl: usize = 0;
        let empty_str = "".to_string();

        for (i, r) in grid.iter_rows().enumerate() {
            let p = prev.last().unwrap_or(&empty_str);
            let s = r.into_iter().collect::<String>();

            if i > refl_min && refl == 0 && &s == p {
                refl = i;
            }

            match refl {
                0 => prev.push(s.clone()),
                _ => next.push(s.clone()),
            }
        }

        prev.reverse();

        for i in 0..cmp::min(prev.len(), next.len()) {
            if prev[i] != next[i] {
                return self.find_reflection_score(&grid, refl);
            }
        }

        refl
    }

    fn find_horizontal_reflection_score(&self, grid: &Grid<char>, refl_min: usize) -> usize {
        self.find_reflection_score(grid, refl_min) * 100
    }

    fn find_vertical_reflection_score(&self, grid: &Grid<char>, refl_min: usize) -> usize {
        let mut mgrid = grid.clone();
        mgrid.transpose();

        self.find_reflection_score(&mgrid, refl_min)
    }

    fn reflection_score(&self) -> usize {
        match self.find_horizontal_reflection_score(&self.grid, 0) {
            0 => self.find_vertical_reflection_score(&self.grid, 0),
            s => s,
        }
    }

    fn reflection_score_without_smudge(&self) -> usize {
        let score = self.reflection_score();
        let swaps = HashMap::from([('.', '#'), ('#', '.')]);

        for ((row, col), val) in self.grid.indexed_iter() {
            let mut grid = self.grid.clone();
            grid[(row, col)] = *swaps.get(val).unwrap();

            let mut score_without_smudge = match self.find_horizontal_reflection_score(&grid, 0) {
                0 => self.find_vertical_reflection_score(&grid, 0),
                s => s,
            };

            // FIXME: This is a hack to avoid the case where the reflection score is calculated before valid reflection without smudge score
            if score_without_smudge == score {
                score_without_smudge = match self.find_horizontal_reflection_score(&grid, score / 100) {
                    0 => self.find_vertical_reflection_score(&grid, score % 100),
                    s => s,
                };
            }

            if score_without_smudge != 0 && score_without_smudge != score {
                return score_without_smudge;
            }
        }

        score
    }
}