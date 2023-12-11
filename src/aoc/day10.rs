use array2d::Array2D;

pub struct Day10;

impl crate::aoc::Compute for Day10 {
    fn compute_part_one(&self, version: String) -> String {
        self.input_load("1".to_string(), version.clone()).solve().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load("2".to_string(), version.clone());
        "TODO".to_string()
    }
}

impl Day10 {
    fn input_load(&self, part: String, version: String) -> PipeMaze {
        PipeMaze { grid: crate::aoc::input_load_as_array2d("10".to_string(), part, version) }
    }
}

struct PipeMaze {
    grid: Array2D<char>,
}

impl PipeMaze {
    fn solve(&self) -> usize {
        let start = self.grid.enumerate_row_major().find(|(_, c)| *c == &'S').map(|(l, _)| Cell { x: l.1, y: l.0 }).unwrap();
        let boundaries = Boundaries { x_max: self.grid.num_columns() - 1, x_min: 0, y_max: self.grid.num_rows() - 1, y_min: 0 };
        let adjacents: Vec<Cell> = boundaries.adjacents(&start);

        let mut forw: PipeMazePath = PipeMazePath { step: 0, cell: start };
        let mut back: PipeMazePath = PipeMazePath { step: 0, cell: start };

        1 as usize
    }

    // fn locate_paths(&self, start: Cell) -> (PipeMazePath, PipeMazePath) {
    //     (forw, back)
    // }
}

struct PipeMazePath {
    step: usize,
    cell: Cell,
}

struct Boundaries {
    x_max: usize,
    x_min: usize,
    y_max: usize,
    y_min: usize
}

impl Boundaries {
    fn adjacents(&self, cell: &Cell) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::new();

        if cell.y > self.y_min { cells.push(Cell {x: cell.x, y: cell.y - 1}) } // north cell
        if cell.x < self.x_max { cells.push(Cell {x: cell.x + 1, y: cell.y}) } // east cell
        if cell.y < self.y_max { cells.push(Cell {x: cell.x, y: cell.y + 1}) } // south cell
        if cell.x > self.x_min { cells.push(Cell {x: cell.x - 1, y: cell.y}) } // west cell

        cells
    }
}

struct Cell {
    x: usize,
    y: usize
}
