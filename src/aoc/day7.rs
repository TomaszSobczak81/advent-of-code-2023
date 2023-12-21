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
    // TODO: refactor this to optimize the code
    // Test#1 OK: Got 6440 as expected
    // Part#1 result: 252052080 computed in 487.763643ms
    // Test#2 OK: Got 5905 as expected
    // Part#2 result: 252898370 computed in 282.341661985s
    fn compute(&self, part: String, version: String) -> String {
        let mut input = self.input_load(part.clone(), version.clone());
        let mut total = 0 as usize;
        
        input.sort_by(|a, b| a.compare_by_type(&b, true));
        
        for (i, hand) in input.iter().enumerate() {
            total += hand.winning_bid * (i + 1);
        }

        total.to_string()
    }

    fn input_load(&self, part: String, version: String) -> Vec<HandOfCards> {
        let input = crate::aoc::input_load("7".to_string(), part.clone(), version.clone()); 
        input.lines().map(|line| {
            let mut parts = line.split(" ");
            HandOfCards::create(
                parts.next().unwrap().trim().chars().collect(),
                part.eq("2"),
                parts.next().unwrap().trim().parse::<usize>().unwrap()
            )
        }).collect()
    }
}

#[derive(Debug)]
struct HandOfCards {
    given_cards: Vec<char>,
    joker_cards: Vec<char>,
    using_joker: bool,
    winning_bid: usize,
}

impl HandOfCards {
    fn create(given_cards: Vec<char>, using_joker: bool, winning_bid: usize) -> Self {
        let mut hoc = Self { 
            given_cards: given_cards, 
            joker_cards: Vec::new(),
            using_joker: using_joker, 
            winning_bid: winning_bid
        };

        if using_joker {
            hoc.joker_cards = hoc.joker_deck();
        }

        hoc
    }

    fn joker_deck(&self) -> Vec<char> {
        if !self.given_cards.contains(&'J') {
            return self.given_cards.clone();
        }

        let cards = self.given_cards.iter().filter(|&c| !c.eq(&'J')).map(|c| *c).collect::<Vec<char>>();
        let faces = Vec::from(['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2']);
        let steps = 5 - cards.len();

        let mut decks = Vec::from([cards.clone()]);

        for _ in 0..steps {
            decks = decks.iter().map(|deck| {
                faces.iter().map(|face| {
                    let mut new_deck = deck.clone();
                    new_deck.push(*face);
                    new_deck
                })
            }).flatten().collect()
        }

        let mut hands = decks.iter().map(|d| HandOfCards::create(d.clone(), false, 0)).collect::<Vec<HandOfCards>>();
        hands.sort_by(|a, b| a.compare_by_type(&b, false));
        hands.iter().last().unwrap().given_cards.clone()
    }

    fn cards(&self) -> std::collections::HashMap<char, usize> {
        let mut cards: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
        let cards_to_look: Vec<char> = if self.using_joker {self.joker_cards.clone()} else {self.given_cards.clone()};

        for card in cards_to_look {
            *cards.entry(card).or_insert(0) += 1;
        }

        cards
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

        if self.cards().values().filter(|&count| count == &2).count() == 2 {
            return "is_two_pairs".to_string();
        }

        if self.is_x_of_a_kind(2) {
            return "is_one_pair".to_string();
        }

        "is_high_card".to_string()
    }

    fn is_x_of_a_kind(&self, x: usize) -> bool {
        self.cards().values().any(|&count| count == x)
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

    fn compare_by_type(&self, other: &HandOfCards, compare_by_cards: bool) -> std::cmp::Ordering {
        if self.cards_strength() > other.cards_strength() {
            return std::cmp::Ordering::Greater;
        } else if self.cards_strength() < other.cards_strength() {
            return std::cmp::Ordering::Less;
        }

        match compare_by_cards {
            true => self.compare_by_cards(other),
            false => std::cmp::Ordering::Equal,
        }
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
