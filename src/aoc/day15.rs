pub struct Day15;

impl crate::aoc::Compute for Day15 {
    fn compute_part_one(&self, version: String) -> String {
        let steps: Vec<String> = self.input_load("1".to_string(), version.clone());
        steps.iter().map(|s| self.hash(s.to_string())).sum::<usize>().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let mut boxes: Vec<Box> = (0..256).map(|i| Box::new(i)).collect();
        let steps: Vec<String> = self.input_load("1".to_string(), version.clone());

        for s in steps.iter() {
            if let Some((label, focal)) = s.split_once("=") {
                boxes[self.hash(label.to_string())].add_or_replace_lens(label.to_string(), focal.parse::<usize>().unwrap());
            } else if let Some((label, _)) = s.split_once("-") {
                boxes[self.hash(label.to_string())].remove_lens(label.to_string());
            } else {
                panic!("Invalid step: {}", s);
            }
        }

        boxes.iter().map(|b| b.focusing_power()).sum::<usize>().to_string()
    }
}

impl Day15 {
    fn hash(&self, input: String) -> usize {
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
        steps.split(",").map(|s| s.trim().to_string()).collect()
    }
}

struct Box {
    boxid: usize,
    slots: Vec<Lens>
}

impl Box {
    fn new(boxid: usize) -> Self {
        Self { boxid, slots: Vec::new() }
    }

    fn add_or_replace_lens(&mut self, label: String, focal: usize) {
        if let Some(index) = self.slots.iter().position(|s| s.label == label) {
            self.slots[index].focal = focal;
        } else {
            self.slots.push(Lens::new(label, focal));
        }
    }

    fn remove_lens(&mut self, label: String) {
        if let Some(index) = self.slots.iter().position(|s| s.label == label) {
            self.slots.remove(index);
        }
    }

    fn focusing_power(&self) -> usize {
        self.slots.iter().enumerate().map(|(i, s)| (1 + self.boxid) * (i + 1) * s.focal).sum::<usize>()
    }
}

struct Lens {
    label: String,
    focal: usize
}

impl Lens {
    fn new(label: String, focal: usize) -> Self {
        Self { label, focal }
    }
}
