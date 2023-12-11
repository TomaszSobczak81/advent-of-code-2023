pub struct Day10;

impl crate::aoc::Compute for Day10 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        "TODO".to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load("2".to_string(), version.clone());
        "TODO".to_string()
    }
}

impl Day10 {
    fn input_load(&self, part: String, version: String) -> String {
        crate::aoc::input_load("10".to_string(), part, version.clone())
    }
}
