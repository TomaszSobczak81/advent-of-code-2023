use crate::aoc::Compass;
use grid::Grid;

pub struct Day16;

impl crate::aoc::Compute for Day16 {
    fn compute_part_one(&self, version: String) -> String {
        let mut contraption = self.input_load("1".to_string(), version.clone());
        contraption.run(Beam::new(Point::new(-1, 0), Compass::EAST));
        contraption.count_energized_tiles().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let contraption = self.input_load("1".to_string(), version.clone());
        let mut results: Vec<usize> = Vec::new();

        for x in 0..contraption.grid.cols() {
            let mut cs = contraption.clone();
            let mut cn = contraption.clone();

            cs.run(Beam::new(Point::new(x as isize, -1), Compass::SOUTH));
            cn.run(Beam::new(Point::new(x as isize, contraption.grid.rows() as isize), Compass::NORTH));

            results.push(cs.count_energized_tiles());
            results.push(cn.count_energized_tiles());
        }

        for y in 0..contraption.grid.rows() {
            let mut cw = contraption.clone();
            let mut ce = contraption.clone();

            ce.run(Beam::new(Point::new(-1, y as isize), Compass::EAST));
            cw.run(Beam::new(Point::new(contraption.grid.cols() as isize, y as isize), Compass::WEST));

            results.push(ce.count_energized_tiles());
            results.push(cw.count_energized_tiles());
        }

        results.iter().max().unwrap_or(&0).to_string()
    }
}

impl Day16 {
    fn input_load(&self, part: String, version: String) -> Contraption {
        let data: Grid<char> = crate::aoc::input_load_as_grid("16".to_string(), part, version);
        Contraption::new(data)
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Beam {
    src_point: Point,
    direction: Compass,
}

impl Beam {
    fn new(src_point: Point, direction: Compass) -> Self {
        Self { src_point, direction }
    }

    fn advance(&mut self, point: Point, direction: Compass) -> Point {
        self.direction = direction;

        match direction {
            Compass::NORTH => Point::new(point.x, point.y - 1),
            Compass::EAST => Point::new(point.x + 1, point.y),
            Compass::SOUTH => Point::new(point.x, point.y + 1),
            Compass::WEST => Point::new(point.x - 1, point.y),
        }
    }

    fn track(&mut self, contraption: &mut Contraption) -> Point {
        let mut curr = self.advance(self.src_point, self.direction);

        loop {
            if !contraption.include(curr) {
                break;
            }

            contraption.energize(curr);

            curr = match contraption.grid[(curr.y as usize, curr.x as usize)] as u32 {
                // '.' as u32
                46 => self.advance(curr, self.direction),
                // '/' as u32
                47 => match self.direction {
                    Compass::NORTH => self.advance(curr, Compass::EAST),
                    Compass::EAST => self.advance(curr, Compass::NORTH),
                    Compass::SOUTH => self.advance(curr, Compass::WEST),
                    Compass::WEST => self.advance(curr, Compass::SOUTH),
                },
                // '\' as u32
                92 => match self.direction {
                    Compass::NORTH => self.advance(curr, Compass::WEST),
                    Compass::EAST => self.advance(curr, Compass::SOUTH),
                    Compass::SOUTH => self.advance(curr, Compass::EAST),
                    Compass::WEST => self.advance(curr, Compass::NORTH),
                },
                // '|' as u32
                124 => match self.direction {
                    Compass::NORTH | Compass::SOUTH => self.advance(curr, self.direction),
                    Compass::EAST | Compass::WEST => break,
                },
                // '-' as u32
                45 => match self.direction {
                    Compass::NORTH | Compass::SOUTH => break,
                    Compass::EAST | Compass::WEST => self.advance(curr, self.direction),
                },
                _ => panic!("Unknown tile {:?} at x={} y={}", contraption.grid[(curr.y as usize, curr.x as usize)], curr.x, curr.y),
            };
        }

        curr
    }
}

#[derive(Clone)]
struct Contraption {
    grid: Grid<char>,
    copy: Grid<char>,
    beams: Vec<Beam>,
}

impl Contraption {
    fn new(grid: Grid<char>) -> Self {
        Self { grid: grid.clone(), copy: grid.clone(), beams: Vec::new()}
    }

    fn count_energized_tiles(&self) -> usize {
        self.copy.iter().filter(|c| **c == '#').count()
    }

    fn energize(&mut self, point: Point) {
        self.copy[(point.y as usize, point.x as usize)] = '#';
    }

    fn include(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.grid.cols() as isize && point.y >= 0 && point.y < self.grid.rows() as isize
    }

    fn run(&mut self, b: Beam) {
        self.track_beam(b);
    }

    fn split_beam(&mut self, p: Point) {
        match self.grid[(p.y as usize, p.x as usize)] as u32 {
            // '|' as u32
            124 =>  {
                self.track_beam(Beam::new(p, Compass::NORTH));
                self.track_beam(Beam::new(p, Compass::SOUTH));
            },
            // '-' as u32
            45 =>  {
                self.track_beam(Beam::new(p, Compass::EAST));
                self.track_beam(Beam::new(p, Compass::WEST));
            },
            _ => println!("Unknown tile {:?} at x={} y={}", self.grid[(p.y as usize, p.x as usize)], p.x, p.y),
        };
    }

    fn track_beam(&mut self, mut beam: Beam) {
        if let Some(_) = self.beams.iter().position(|b| b.src_point == beam.src_point && b.direction == beam.direction) {
            return; // Beam already tracked
        }

        self.beams.push(beam.clone());

        let p = beam.track(self);

        match self.include(p) {
            true => self.split_beam(p),
            false => return,
        };
    }
}