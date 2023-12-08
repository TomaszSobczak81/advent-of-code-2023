pub struct Day3;

impl crate::aoc::Compute for Day3 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        let parts = self.collect_parts(&input);
        let boundaries = Boundaries { x_max: input.num_columns() - 1, x_min: 0, y_max: input.num_rows() - 1, y_min: 0 };
        let mut sum: i32 = 0;

        for part in parts.iter() {
            let adjacents: Vec<char> = boundaries.adjacent_cells_for_part(&part).iter().map(|c| input[((*c).y, (*c).x)]).collect();
            
            match adjacents.iter().any(|c| !c.is_digit(10) && *c != '.') {
                true => sum += part.value,
                false => (),
            }
        }

        sum.to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load("2".to_string(), version.clone());
        let parts = self.collect_parts(&input);
        let boundaries = Boundaries { x_max: input.num_columns() - 1, x_min: 0, y_max: input.num_rows() - 1, y_min: 0 };
        let mut gears = array2d::Array2D::filled_with(Vec::<i32>::new(), input.num_rows(), input.num_columns());
        let mut sum: i32 = 0;

        for part in parts.iter() {
            let adjacents: Vec<Cell> = boundaries.adjacent_cells_for_part(&part);

            for cell in adjacents.iter() {
                if input[((*cell).y, (*cell).x)] == '*' {
                    gears[((*cell).y, (*cell).x)].push(part.value);
                    break;
                }
            }
        }

        for r in 0..gears.num_rows() {
            for c in 0..gears.num_columns() {
                if 2 == gears[(r,c)].len() { sum += gears[(r,c)][0] * gears[(r,c)][1] }
            }
        }

        sum.to_string()
    }
}

impl Day3 {
    fn collect_parts(&self, grid: &array2d::Array2D<char>) -> Vec<Part> {
        let mut list: Vec<Part> = Vec::new();

        for r in 0..grid.num_rows() {
            let mut value: String = "".to_string();
            let mut cells: Vec<Cell> = Vec::new();

            for c in 0..grid.num_columns() {
                let v: char = grid[(r,c)];

                if v.is_digit(10) {
                    value.push(v);
                    cells.push(Cell { x: c, y: r });
                } else if value != "" {
                    list.push(Part { value: value.parse::<i32>().unwrap(), cells: cells });
                    value = "".to_string();
                    cells = Vec::new();
                }
            }

            if value != "" {
                list.push(Part { value: value.parse::<i32>().unwrap(), cells: cells });
            }
        }

        list
    }

    fn input_load(&self, part: String, version: String) -> array2d::Array2D<char> {
        let input = crate::aoc::input_load("3".to_string(), part, version.clone());
        array2d::Array2D::from_rows(&input.lines().map(|l| l.chars().collect()).collect::<Vec<_>>()).unwrap()
    }
}

struct Boundaries {
    x_max: usize,
    x_min: usize,
    y_max: usize,
    y_min: usize
}

impl Boundaries {
    fn adjacent_cells_for_cell(&self, cell: &Cell) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::new();

        if cell.y > self.y_min { cells.push(Cell {x: cell.x, y: cell.y - 1}) } // north cell
        if cell.y > self.y_min && cell.x < self.x_max { cells.push(Cell {x: cell.x + 1, y: cell.y - 1}) } // north-east cell
        if cell.x < self.x_max { cells.push(Cell {x: cell.x + 1, y: cell.y}) } // east cell
        if cell.y < self.y_max && cell.x < self.x_max { cells.push(Cell {x: cell.x + 1, y: cell.y + 1}) } // south-east cell
        if cell.y < self.y_max { cells.push(Cell {x: cell.x, y: cell.y + 1}) } // south cell
        if cell.y < self.y_max && cell.x > self.x_min { cells.push(Cell {x: cell.x - 1, y: cell.y + 1}) } // south-west cell
        if cell.x > self.x_min { cells.push(Cell {x: cell.x - 1, y: cell.y}) } // west cell
        if cell.y > self.y_min && cell.x > self.x_min { cells.push(Cell {x: cell.x - 1, y: cell.y - 1}) } // north-west cell

        cells
    }

    fn adjacent_cells_for_part(&self, part: &Part) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::new();

        for cell in part.cells.iter() {
            cells.append(&mut self.adjacent_cells_for_cell(cell));
        }

        cells
    }
}

struct Cell {
    x: usize,
    y: usize
}

struct Part {
    value: i32,
    cells: Vec<Cell>
}
