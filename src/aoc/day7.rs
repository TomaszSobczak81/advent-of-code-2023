pub struct Day7;

impl crate::aoc::Compute for Day7 {
    fn compute_part_one(&self, version: String) -> String {
        self.compute("1".to_string(), version.clone())
    }

    fn compute_part_two(&self, version: String) -> String {
        self.compute("2".to_string(), version.clone())
    }
}

impl Day7 {
    fn compute(&self, part: String, version: String) -> String {
        let mut input = self.input_load(part.clone(), version.clone());
        let mut total = 0 as usize;
        
        input.sort_by(|a, b| a.compare_by_type(&b));
        
        for (i, hand) in input.iter().enumerate() {
            total += hand.winning_bid * (i + 1);
        }

        total.to_string()
    }

    fn input_load(&self, part: String, version: String) -> Vec<HandOfCards> {
        let input = crate::aoc::input_load("7".to_string(), part.clone(), version.clone()); 
        input.lines().map(|line| {
            let mut parts = line.split(" ");
            HandOfCards {
                given_cards: parts.next().unwrap().trim().chars().collect(),
                using_joker: part.eq("2"),
                winning_bid: parts.next().unwrap().trim().parse::<usize>().unwrap(),
            }
        }).collect()
    }
}

#[derive(Debug)]
struct HandOfCards {
    given_cards: Vec<char>,
    using_joker: bool,
    winning_bid: usize,
}

impl HandOfCards {
    fn joker_cards(&self) -> Vec<char> {
        if self.given_cards.contains(&'J') {
            let mut cards_map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
            // let cards_new = self.given_cards.clone();

            for card in &self.given_cards {
                *cards_map.entry(*card).or_insert(0) += 1;
            }

            // promote four of a kind up
            if cards_map.values().any(|&count| count == 4) {
                let card_to_promote = cards_map.iter().find(|&(_, &count)| count == 4).unwrap().0;
                return self.given_cards.iter().map(|c| if (*c).eq(&'J') {*card_to_promote} else {*c}).collect::<Vec<char>>();
            }

            // promote three of a kind up
            if cards_map.values().any(|&count| count == 3) {
                let card_to_promote = cards_map.iter().find(|&(_, &count)| count == 3).unwrap().0;
                return self.given_cards.iter().map(|c| if (*c).eq(&'J') {*card_to_promote} else {*c}).collect::<Vec<char>>();
            }

            println!("{:?}", self.given_cards);
            return Vec::new();
        }

        self.given_cards.clone()
    }

    fn cards_map(&self) -> std::collections::HashMap<char, usize> {
        let mut cards_map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
        let cards_to_look: Vec<char> = if self.using_joker {self.joker_cards()} else {self.given_cards.clone()};

        for card in cards_to_look {
            *cards_map.entry(card).or_insert(0) += 1;
        }

        cards_map
    }

    fn hand_type(&self) -> String {
        if self.is_x_of_a_kind(5) {
            return "is_five_of_a_kind".to_string();
        }

        if self.is_x_of_a_kind(4) {
            return "is_four_of_a_kind".to_string();
        }

        if self.is_x_of_a_kind(3) && self.is_x_of_a_kind(2) {
            return "is_full_house".to_string();
        }

        if self.is_x_of_a_kind(3) {
            return "is_three_of_a_kind".to_string();
        }

        if self.cards_map().values().filter(|&count| count == &2).count() == 2 {
            return "is_two_pairs".to_string();
        }

        if self.is_x_of_a_kind(2) {
            return "is_one_pair".to_string();
        }

        "is_high_card".to_string()
    }

    fn is_x_of_a_kind(&self, x: usize) -> bool {
        self.cards_map().values().any(|&count| count == x)
    }

    fn cards_strength(&self) -> usize {
        match self.hand_type().as_str() {
            "is_five_of_a_kind" => 7,
            "is_four_of_a_kind" => 6,
            "is_full_house" => 5,
            "is_three_of_a_kind" => 4,
            "is_two_pairs" => 3,
            "is_one_pair" => 2,
            "is_high_card" => 1,
            _ => 0,
        }
    }

    fn compare_by_type(&self, other: &HandOfCards) -> std::cmp::Ordering {
        if self.cards_strength() > other.cards_strength() {
            return std::cmp::Ordering::Greater;
        } else if self.cards_strength() < other.cards_strength() {
            return std::cmp::Ordering::Less;
        }

        self.compare_by_cards(other)
    }

    fn compare_by_cards(&self, other: &HandOfCards) -> std::cmp::Ordering {
        for (self_card, other_card) in self.given_cards.iter().zip(other.given_cards.iter()) {
            if self.card_score(*self_card) > self.card_score(*other_card) {
                return std::cmp::Ordering::Greater;
            } else if self.card_score(*self_card) < self.card_score(*other_card) {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    }

    fn card_score(&self, card: char) -> usize {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => if self.using_joker { 1 } else { 11 },
            'T' => 10,
            _ => card.to_digit(10).unwrap() as usize,
        }
    }    
}
