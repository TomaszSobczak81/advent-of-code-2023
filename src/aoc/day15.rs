pub struct Day15;

impl crate::aoc::Compute for Day15 {
    fn compute_part_one(&self, version: String) -> String {
        let steps: Vec<String> = self.input_load("1".to_string(), version.clone());
        steps.iter().map(|s| self.hash_string(s.to_string())).sum::<usize>().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        "TODO".to_string()
    }
}

impl Day15 {
    fn hash_string(&self, input: String) -> usize {
        let mut hash = 0;

        for c in input.chars() {
            hash += c as u32;
            hash *= 17;
            hash %= 256;
        }

        hash as usize
    }

    fn input_load(&self, part: String, version: String) -> Vec<String> {
        let steps: String = crate::aoc::input_load("15".to_string(), part, version);
        steps.split(",").map(|c| c.trim().to_string()).collect()
    }
}
