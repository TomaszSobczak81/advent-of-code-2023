use std::collections::HashMap;

pub struct Day4;

impl crate::aoc::Compute for Day4 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        let sum: i32 = input.iter().map(|c| c.score()).sum();

        sum.to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load("2".to_string(), version.clone());
        let mut cards = HashMap::from(input.iter().map(|c| (c.card_id, 1)).collect::<HashMap<i32, i32>>());

        for card in input.iter() {
            if !card.has_winning_numbers() { continue; }

            let intersections = card.intersection().len() as i32;
            let copies_number = cards[&card.card_id];
            for id in card.card_id + 1..=card.card_id + intersections {
                if let Some(c) = cards.get_mut(&id) {
                    *c = *c + copies_number;
                }
            }
        }

        cards.values().sum::<i32>().to_string()
    }
}

impl Day4 {
    fn input_load(&self, part: String, version: String) -> Vec<Card> {
        let input = crate::aoc::input_load("4".to_string(), part, version.clone());
        input.lines().map(|line| self.parse_card_info(line)).collect()
    }

    fn parse_card_info(&self, line: &str) -> Card {
        let mut parts = line.split(":");
        let card_id: String = parts.next().unwrap().chars().filter(|c| c.is_digit(10)).collect();
        let subsets: Vec<String> = parts.next().unwrap().split("|").map(|s| s.trim().to_string()).collect();

        Card {
            card_id: card_id.parse::<i32>().unwrap(),
            numbers_on_card: subsets[1].split(" ").filter(|&s| !s.is_empty()).map(|n| n.trim().parse::<i32>().unwrap()).collect(),
            numbers_winning: subsets[0].split(" ").filter(|&s| !s.is_empty()).map(|n| n.trim().parse::<i32>().unwrap()).collect()
        }
    }
}

#[derive(Debug)]
struct Card {
    card_id: i32,
    numbers_on_card: Vec<i32>,
    numbers_winning: Vec<i32>
}

impl Card {
    fn has_winning_numbers(&self) -> bool {
        self.intersection().len() > 0
    }

    fn intersection(&self) -> Vec<i32> {
        self.numbers_on_card.iter().filter(|n| self.numbers_winning.contains(n)).map(|n| *n).collect()
    }

    fn score(&self) -> i32 {
        match self.intersection().len() {
            0 => 0,
            _ => 2_i32.pow(self.intersection().len() as u32 - 1)
        }        
    }
}
