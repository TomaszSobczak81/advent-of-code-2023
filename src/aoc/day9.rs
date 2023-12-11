pub struct Day9;

impl crate::aoc::Compute for Day9 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        input.solve(false).to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load("2".to_string(), version.clone());
        input.solve(true).to_string()
    }
}

impl Day9 {
    fn input_load(&self, part: String, version: String) -> OASIS {
        let input = crate::aoc::input_load("9".to_string(), part, version.clone());

        OASIS {
            sensors: input.lines().map(|line| {
                Sensor {
                    history: line.split(" ").map(|s| s.parse::<isize>().unwrap()).collect::<Vec<isize>>(),
                }
            }).collect()
        }
    }
}

struct Sensor {
    history: Vec<isize>,
}

impl Sensor {
    fn extrapolate(&self, backward: bool) -> isize {
        self.reduce(self.history.clone(), backward)
    }

    fn reduce(&self, dataset: Vec<isize>, backward: bool) -> isize {
        if dataset.iter().all(|&x| x == 0) {
            return 0 as isize;
        }

        let mut reduced: Vec<isize> = Vec::new();

        for i in 0..dataset.len() - 1 {
            reduced.push(dataset[i + 1] - dataset[i]);
        }

        match backward {
            true => dataset.first().unwrap() - self.reduce(reduced.clone(), backward),
            false => dataset.last().unwrap() + self.reduce(reduced.clone(), backward),
        }
    }
}

struct OASIS {
    sensors: Vec<Sensor>,
}

impl OASIS {
    fn solve(&self, backward: bool) -> isize {
        self.sensors.iter().map(|s| s.extrapolate(backward)).sum()
    }
}
