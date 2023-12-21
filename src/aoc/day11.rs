use grid::Grid;

pub struct Day11;

impl crate::aoc::Compute for Day11 {
    fn compute_part_one(&self, version: String) -> String {
        let universe = self.input_load("1".to_string(), version.clone());
        universe.overal_steps(2 as usize).to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let universe = self.input_load("1".to_string(), version.clone());
        universe.overal_steps(1000000 as usize).to_string()
    }
}

impl Day11 {
    // TODO: refactor this to optimize the code
    // Test#1 OK: Got 374 as expected
    // Part#1 result: 9233514 computed in 17.144688429s
    // Test#2 OK: Got 82000210 as expected
    // Part#2 result: 363293506944 computed in 17.102510484s
    fn input_load(&self, part: String, version: String) -> Universe {
        Universe::new(crate::aoc::input_load_as_grid("11".to_string(), part, version))
    }
}

struct Universe {
    grid: Grid<char>,
}

impl Universe {
    fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        self.grid.indexed_iter().filter(|(_, item)| **item == '#').map(|(loc, _)| loc).collect::<Vec<(usize, usize)>>()
    }

    fn pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut pairs = vec![];

        for (i, (y1, x1)) in self.galaxies().iter().enumerate() {
            for (j, (y2, x2)) in self.galaxies().iter().enumerate() {
                if i < j {
                    pairs.push(((*x1, *y1), (*x2, *y2)));
                }
            }
        }

        pairs
    }

    fn overal_steps(&self, galaxy_expansion: usize) -> usize {
        let mut steps = 0;

        for pair in self.pairs() {
            steps += self.steps_between_pair(pair, galaxy_expansion);
        }

        steps
    }

    fn steps_between_pair(&self, pair: ((usize, usize), (usize, usize)), galaxy_expansion: usize) -> usize {
        let mut steps = 0;

        let (x1, y1) = pair.0;
        let (x2, y2) = pair.1;

        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            if x != x2 {
                x = if x < x2 { x + 1 } else { x - 1 };
                if self.grid.iter_col(x).clone().filter(|c| **c == '#').collect::<Vec<&char>>().is_empty() {
                    steps += galaxy_expansion;
                } else {
                    steps += 1;
                }
            }

            if y != y2 {
                y = if y < y2 { y + 1 } else { y - 1 };
                if self.grid.iter_row(y).clone().filter(|c| **c == '#').collect::<Vec<&char>>().is_empty() {
                    steps += galaxy_expansion;
                } else {
                    steps += 1;
                }
            }
        }

        steps
    }
}