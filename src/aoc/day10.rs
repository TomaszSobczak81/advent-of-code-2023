use array2d::Array2D;

pub struct Day10;

impl crate::aoc::Compute for Day10 {
    fn compute_part_one(&self, version: String) -> String {
        let mut input = self.input_load("1".to_string(), version.clone());
        self.solve_maze(&mut input).to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        "TODO".to_string()
    }
}

impl Day10 {
    fn input_load(&self, part: String, version: String) -> Array2D<char> {
        crate::aoc::input_load_as_array2d("10".to_string(), part, version)
    }

    fn solve_maze(&self, input: &mut Array2D<char>) -> usize {
        let start = input.enumerate_row_major().find(|(_, c)| *c == &'S').map(|(l, _)| l).unwrap();
        let ppath = PipePath { step: 0, curr: Cell { c: input[(start.0, start.1)], x: start.1, y: start.0 }, prev: Cell { c: input[(start.0, start.1)], x: start.1, y: start.0 } };
        input[(ppath.curr.y, ppath.curr.x)] = 'X';
        self.process_path(input, ppath).step / 2 as usize + 1
    }

    fn adjacents(&self, grid: &Array2D<char>, cell: &Cell) -> Vec<Cell> {
        let mut adjacents: Vec<Cell> = Vec::new();

        if cell.y > 0 { adjacents.push(Cell { c: grid[(cell.y - 1, cell.x)], x: cell.x, y: cell.y - 1 }) } // north cell
        if cell.x < grid.num_columns() - 1 { adjacents.push(Cell { c: grid[(cell.y, cell.x + 1)], x: cell.x + 1, y: cell.y }) } // east cell
        if cell.y < grid.num_rows() - 1 { adjacents.push(Cell { c: grid[(cell.y + 1, cell.x)], x: cell.x, y: cell.y + 1 }) } // south cell
        if cell.x > 0 { adjacents.push(Cell { c: grid[(cell.y, cell.x - 1)], x: cell.x - 1, y: cell.y }) } // west cell

        adjacents
    }

    fn process_path(&self, grid: &mut Array2D<char>, path: PipePath) -> PipePath {
        let mut moved = false;
        let mut ppath = path.clone();

        for c in self.adjacents(grid, &ppath.curr) {
            (moved, ppath) = path.next(c.clone());
            if moved { break }
        }

        match moved {
            true => self.process_path(grid, ppath),
            false => ppath
        }
    }
}

#[derive(Clone,Debug)]
struct Cell {
    c: char,
    x: usize,
    y: usize
}

impl Cell {
    fn can_access_north(&self) -> bool {
        match self.c {
            '|' | 'L' | 'J' | 'S' => true,
            _ => false
        }
    }

    fn can_access_east(&self) -> bool {
        match self.c {
            '-' | 'F' | 'L' | 'S' => true,
            _ => false
        }
    }

    fn can_access_south(&self) -> bool {
        match self.c {
            '|' | '7' | 'F' | 'S' => true,
            _ => false
        }
    }

    fn can_access_west(&self) -> bool {
        match self.c {
            '-' | 'J' | '7' | 'S' => true,
            _ => false
        }
    }

    fn can_accept_north(&self) -> bool {
        match self.c {
            '|' | 'J' | 'L' => true,
            _ => false
        }
    }

    fn can_accept_east(&self) -> bool {
        match self.c {
            '-' | 'F' | 'L' => true,
            _ => false
        }
    }

    fn can_accept_south(&self) -> bool {
        match self.c {
            '|' | '7' | 'F' => true,
            _ => false
        }
    }

    fn can_accept_west(&self) -> bool {
        match self.c {
            '-' | 'J' | '7' => true,
            _ => false
        }
    }
}

#[derive(Clone, Debug)]
struct PipePath {
    step: usize,
    curr: Cell,
    prev: Cell
}

impl PipePath {
    fn next(&self, next: Cell) -> (bool,PipePath) {
        if self.prev.x == next.x && self.prev.y == next.y {
            return (false, self.clone());
        }

        let mut allow_next_step = false;

        // step to north
        if self.curr.x == next.x && self.curr.y > 0 && self.curr.y - 1 == next.y {
            allow_next_step = self.curr.can_access_north() && next.can_accept_south();
        }

        // step to east
        if self.curr.x + 1 == next.x && self.curr.y == next.y {
            allow_next_step = self.curr.can_access_east() && next.can_accept_west();
        }

        // step to south
        if self.curr.x == next.x && self.curr.y + 1 == next.y {
            allow_next_step = self.curr.can_access_south() && next.can_accept_north();
        }

        // step to west
        if self.curr.x > 0 && self.curr.x - 1 == next.x && self.curr.y == next.y {
            allow_next_step = self.curr.can_access_west() && next.can_accept_east();
        }

        match allow_next_step {
            true => (true, PipePath { step: self.step + 1, curr: next, prev: self.curr.clone() }),
            false => (false, self.clone())
        }
    }
}
