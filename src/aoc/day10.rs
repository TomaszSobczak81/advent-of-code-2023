use array2d::Array2D;

pub struct Day10;

impl crate::aoc::Compute for Day10 {
    fn compute_part_one(&self, version: String) -> String {
        let mut input = self.input_load("1".to_string(), version.clone());
        self.solve_maze(&mut input).to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let mut input = self.input_load("2".to_string(), version.clone());
        self.solve_maze(&mut input);
        self.find_nest(input).to_string()
    }
}

impl Day10 {
    fn input_load(&self, part: String, version: String) -> Array2D<char> {
        crate::aoc::input_load_as_array2d("10".to_string(), part, version)
    }

    fn find_nest(&self, mut input: Array2D<char>) -> usize {
        let num_columns = input.num_columns() - 1;
        let num_rows = input.num_rows() - 1;

        for (i,r) in input.as_rows().iter().enumerate() {
            for (j,c) in r.iter().enumerate() {
                if *c == 'X' {
                    break;
                }

                input[(i, j)] = '~';
            }

            for (j,c) in r.iter().rev().enumerate() {
                if *c == 'X' {
                    break;
                }

                input[(i, num_columns - j)] = '~';
            }
        }

        for (i,c) in input.as_columns().iter().enumerate() {
            for (j,r) in c.iter().enumerate() {
                if *r == 'X' {
                    break;
                }

                input[(j, i)] = '~';
            }

            for (j,r) in c.iter().rev().enumerate() {
                if *r == 'X' {
                    break;
                }

                input[(num_rows - j, i)] = '~';
            }
        }

        // let cells = input.clone().enumerate_row_major().filter(|(_, v)| !"~X".contains(&v.to_string()));
        // self.process_nest_leftovers(&mut input, cells);

        loop {
            let binds = input.clone();
            let cells = binds.enumerate_row_major().filter(|(_, v)| !"~X".contains(&v.to_string()));
            let count = cells.clone().count();

            for (pos, val) in cells {
                if self.adjacents_diagonal(&input, &Cell { c: *val, x: pos.1, y: pos.0 }).iter().any(|c| c.c == '~') {
                    input[pos] = '~';
                }
            }

            if count == input.enumerate_row_major().filter(|(_, v)| !"~X".contains(&v.to_string())).count() {
                break;
            }
        }

        for r in input.as_rows() {
            println!("{:?}", r.iter().collect::<String>());
        }

        input.enumerate_row_major().filter(|(_, v)| !"~X".contains(&v.to_string())).count()        
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

    fn adjacents_diagonal(&self, grid: &Array2D<char>, cell: &Cell) -> Vec<Cell> {
        let mut adjacents: Vec<Cell> = self.adjacents(grid, cell);

        if cell.y > 0 && cell.x < grid.num_columns() - 1 { adjacents.push(Cell { c: grid[(cell.y - 1, cell.x + 1)], x: cell.x + 1, y: cell.y - 1 }) } // north-east cell
        if cell.y < grid.num_rows() - 1 && cell.x < grid.num_columns() - 1 { adjacents.push(Cell { c: grid[(cell.y + 1, cell.x + 1)], x: cell.x + 1, y: cell.y + 1 }) } // south-east cell
        if cell.y < grid.num_rows() - 1 && cell.x > 0 { adjacents.push(Cell { c: grid[(cell.y + 1, cell.x - 1)], x: cell.x - 1, y: cell.y + 1 }) } // south-west cell
        if cell.y > 0 && cell.x > 0 { adjacents.push(Cell { c: grid[(cell.y - 1, cell.x - 1)], x: cell.x - 1, y: cell.y - 1 }) } // north-west cell

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
            true => {
                grid[(ppath.curr.y, ppath.curr.x)] = 'X';
                self.process_path(grid, ppath)
            },
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
