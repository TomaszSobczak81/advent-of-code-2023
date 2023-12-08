use std::collections::LinkedList;

pub struct Day3;

impl crate::aoc::Compute for Day3 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        let parts = self.collect_parts(&input);
        let sum: i32 = parts.iter().map(|part| part.number(&input)).sum();

        sum.to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load("2".to_string(), version.clone());
        "0".to_string()
    }
}

impl Day3 {
    fn collect_parts(&self, grid: &array2d::Array2D<char>) -> LinkedList<Part> {
        let mut list: LinkedList<Part> = LinkedList::new();

        for r in 0..grid.num_rows() {
            let mut value: String = "".to_string();
            let mut cells: LinkedList<Cell> = LinkedList::new();

            for c in 0..grid.num_columns() {
                let v: char = grid[(r,c)];

                if v.is_digit(10) {
                    value.push(v);
                    cells.push_back(Cell { x: c, y: r });
                } else if value != "" {
                    list.push_back(Part { value: value.parse::<i32>().unwrap(), cells: cells });
                    value = "".to_string();
                    cells = LinkedList::new();                
                }
            }

            if value != "" {
                list.push_back(Part { value: value.parse::<i32>().unwrap(), cells: cells });
            }
        }

        list
    }

    fn input_load(&self, part: String, version: String) -> array2d::Array2D<char> {
        let input = crate::aoc::input_load("3".to_string(), part, version.clone());
        array2d::Array2D::from_rows(&input.lines().map(|l| l.chars().collect()).collect::<Vec<_>>()).unwrap()
    }
}

struct Cell {
    x: usize,
    y: usize
}

struct Part {
    value: i32,
    cells: LinkedList<Cell>
}

impl Part {
    fn adjacent_values(&self, grid: &array2d::Array2D<char>) -> Vec<char> {
        let mut adjacent_values: Vec<char> = Vec::new();

        for cell in self.cells.iter() {
            if cell.y > 0 { // north cell
                adjacent_values.push(grid[(cell.y - 1, cell.x)]);
            }

            if cell.y > 0 && cell.x < grid.num_columns() - 1 { // north-east cell
                adjacent_values.push(grid[(cell.y - 1, cell.x + 1)]);
            }

            if cell.x < grid.num_columns() - 1 { // east cell
                adjacent_values.push(grid[(cell.y, cell.x + 1)]);
            }

            if cell.y < grid.num_rows() - 1 && cell.x < grid.num_columns() - 1 { // south-east cell
                adjacent_values.push(grid[(cell.y + 1, cell.x + 1)]);
            }

            if cell.y < grid.num_rows() - 1 { // south cell
                adjacent_values.push(grid[(cell.y + 1, cell.x)]);
            }

            if cell.y < grid.num_rows() - 1 && cell.x > 0 { // south-west cell
                adjacent_values.push(grid[(cell.y + 1, cell.x - 1)]);
            }

            if cell.x > 0 { // west cell
                adjacent_values.push(grid[(cell.y, cell.x - 1)]);
            }

            if cell.y > 0 && cell.x > 0 { // north-west cell
                adjacent_values.push(grid[(cell.y - 1, cell.x - 1)]);
            }
        }

        adjacent_values
    }

    fn is_adjacent(&self, grid: &array2d::Array2D<char>) -> bool {
        self.adjacent_values(grid).iter().any(|c| !c.is_digit(10) && *c != '.')
    }

    fn is_part_of_gear(&self, grid: &array2d::Array2D<char>) -> bool {
        self.adjacent_values(grid).iter().any(|c| !c.is_digit(10) && *c == '*')
    }

    fn number(&self, grid: &array2d::Array2D<char>) -> i32 {
        match self.is_adjacent(grid) {
            true => self.value,
            false => 0,
        }
    }
}