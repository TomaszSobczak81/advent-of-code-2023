pub struct Day6;

impl crate::aoc::Compute for Day6 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        input.iter().map(|r| r.count_possible_solutions()).reduce(|a, b| a * b).unwrap().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load_merged("2".to_string(), version.clone());
        input.count_possible_solutions().to_string()
    }
}

impl Day6 {
    fn input_load(&self, part: String, version: String) -> Vec<Race> {
        let input = crate::aoc::input_load("6".to_string(), part, version.clone()); 
        let mut races = Vec::new();

        let mut time_values: Vec<usize> = Vec::new();
        let mut dist_values: Vec<usize> = Vec::new();

        for line in input.lines() {
            if line.contains("Time:") {
                let parts = line.split(":");
                time_values = parts.last().unwrap().split(" ").filter(|&s| !s.is_empty()).map(|n| n.trim().parse::<usize>().unwrap()).collect();
            }

            if line.contains("Distance:") {
                let parts = line.split(":");
                dist_values = parts.last().unwrap().split(" ").filter(|&s| !s.is_empty()).map(|n| n.trim().parse::<usize>().unwrap()).collect();
            }
        }

        for i in 0..time_values.len() {
            races.push(Race {
                time_limit: time_values[i],
                distance_to_beat: dist_values[i]
            })
        }

        races
    }

    fn input_load_merged(&self, part: String, version: String) -> Race {
        let input = crate::aoc::input_load("6".to_string(), part, version.clone()); 
        let mut race = Race {
            time_limit: 0,
            distance_to_beat: 0
        };

        for line in input.lines() {
            if line.contains("Time:") {
                let parts = line.split(":");
                race.time_limit = parts.last().unwrap().replace(" ", "").trim().parse::<usize>().unwrap();
            }

            if line.contains("Distance:") {
                let parts = line.split(":");
                race.distance_to_beat = parts.last().unwrap().replace(" ", "").trim().parse::<usize>().unwrap();
            }
        }

        race
    }
}

struct Race {
    time_limit: usize,
    distance_to_beat: usize
}

impl Race {
    fn count_possible_solutions(&self) -> usize {
        let mut solutions = 0;

        for i in 1..self.time_limit {
            let distance = (self.time_limit - i) * i;
            if distance > self.distance_to_beat {
                solutions += 1;
            }
        }

        solutions
    }
}