use grid::Grid;

pub struct Day14;

impl crate::aoc::Compute for Day14 {
    fn compute_part_one(&self, version: String) -> String {
        let mut dish = self.input_load("1".to_string(), version.clone());
        dish.tilt_north();
        dish.total_load().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        "TODO".to_string()
    }
}

impl Day14 {
    fn input_load(&self, part: String, version: String) -> Dish {
        Dish::new(crate::aoc::input_load_as_grid("14".to_string(), part, version))
    }
}

struct Dish {
    grid: Grid<char>
}

impl Dish {
    fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    fn roll_the_rocks(&self, before: Grid<char>) -> Grid<char> {
        let mut after = before.clone();

        for ((row, col), _) in before.indexed_iter().filter(|((_, c), v)| c > &0 && *v == &'O') {
            for i in (1..=col).rev() {
                match after[(row, i - 1)] {
                    '.' => {
                        after[(row, i - 1)] = 'O';
                        after[(row, i)] = '.';
                    },
                    _ => break,
                }
            }
        }

        after
    }

    fn tilt_north(&mut self) {
        self.grid.rotate_left();
        self.grid = self.roll_the_rocks(self.grid.clone());
        self.grid.rotate_right();
    }

    fn total_load(&self) -> usize {
        let mut grid = self.grid.clone();
        let mut load: usize = 0;

        grid.flip_rows();

        for (i, r) in grid.iter_rows().enumerate() {
            load += r.filter(|c| *c == &'O').count() * (i + 1);
        }

        load
    }
}
