use std::collections::LinkedList;

pub struct Day3;

impl crate::aoc::Compute for Day3 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        let parts = self.collect_parts(&input);
        let mut sum: i32 = 0;

        for part in parts.iter() {
            let adjacents: Vec<char> = part.adjacent_cells().iter().map(|c| input[((*c).y, (*c).x)]).collect();
            
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
        let mut sum: i32 = 0;

        for part in parts.iter() {
            let adjacents: Vec<(&Cell, char)> = part.adjacent_cells().iter().map(|c| (c, input[((*c).y, (*c).x)])).collect();

            for a in adjacents.iter() {
                println!("{:?}", a);
            }
            // println!("{:?}", adjacents);
            
            // match adjacents.iter().any(|c| !c.is_digit(10) && *c == '*') {
            //     true => sum += part.value,
            //     false => (),
            // }
        }

        sum.to_string()
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
                    list.push_back(Part { value: value.parse::<i32>().unwrap(), cells: cells, xymax: Cell { x: grid.num_columns() - 1, y: grid.num_rows() - 1 } });
                    value = "".to_string();
                    cells = LinkedList::new();                
                }
            }

            if value != "" {
                list.push_back(Part { value: value.parse::<i32>().unwrap(), cells: cells, xymax: Cell { x: grid.num_columns() - 1, y: grid.num_rows() - 1 } });
            }
        }

        list
    }

    fn input_load(&self, part: String, version: String) -> array2d::Array2D<char> {
        let input = crate::aoc::input_load("3".to_string(), part, version.clone());
        array2d::Array2D::from_rows(&input.lines().map(|l| l.chars().collect()).collect::<Vec<_>>()).unwrap()
    }
}

#[derive(Debug)]
struct Cell {
    x: usize,
    y: usize
}

struct Part {
    value: i32,
    cells: LinkedList<Cell>,
    xymax: Cell
}

impl Part {
    fn adjacent_cells(&self) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::new();

        for cell in self.cells.iter() {
            if cell.y > 0 { // north cell
                cells.push(Cell {x: cell.x, y: cell.y - 1});
            }

            if cell.y > 0 && cell.x < self.xymax.x { // north-east cell
                cells.push(Cell {x: cell.x + 1, y: cell.y - 1});
            }

            if cell.x < self.xymax.x { // east cell
                cells.push(Cell {x: cell.x + 1, y: cell.y});
            }

            if cell.y < self.xymax.y && cell.x < self.xymax.x { // south-east cell
                cells.push(Cell {x: cell.x + 1, y: cell.y + 1});
            }

            if cell.y < self.xymax.y { // south cell
                cells.push(Cell {x: cell.x, y: cell.y + 1});
            }

            if cell.y < self.xymax.y && cell.x > 0 { // south-west cell
                cells.push(Cell {x: cell.x - 1, y: cell.y + 1});
            }

            if cell.x > 0 { // west cell
                cells.push(Cell {x: cell.x - 1, y: cell.y});
            }

            if cell.y > 0 && cell.x > 0 { // north-west cell
                cells.push(Cell {x: cell.x - 1, y: cell.y - 1});
            }
        }

        cells
    }
}