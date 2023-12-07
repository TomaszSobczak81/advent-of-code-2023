pub struct Day2;

impl crate::aoc::Compute for Day2 {
    fn compute_part_one(&self, version: String) -> String {
        let input = self.input_load("1".to_string(), version.clone());
        let mut sum: i32 = 0;

        for game in input.iter() {
            if game.is_valid() {
                sum += game.game_id;
            }
        }

        sum.to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let input = self.input_load("2".to_string(), version.clone());
        let mut sum: i32 = 0;

        for game in input.iter() {
            sum += game.power();
        }

        sum.to_string()
    }
}

impl Day2 {
    fn input_load(&self, part: String, version: String) -> std::collections::LinkedList<Game> {
        let input = crate::aoc::input_load("2".to_string(), part, version.clone());
        let games: std::collections::LinkedList<Game> = input.lines().map(|line| self.parse_game_info(line)).collect();

        games
    }

    fn parse_game_info(&self, line: &str) -> Game {
        let mut parts = line.split(":");
        let game_id: String = parts.next().unwrap().chars().filter(|c| c.is_digit(10)).collect();
        let subsets: Vec<String> = parts.next().unwrap().split(";").map(|s| s.trim().to_string()).collect();

        let mut game = Game {
            game_id: game_id.parse::<i32>().unwrap(),
            r_cubes: 0,
            g_cubes: 0,
            b_cubes: 0,
        };

        for set in subsets.iter() {
            let cubes: Vec<String> = set.split(",").map(|c| c.trim().to_string()).collect();

            for cube in cubes.iter() {
                if let Some((count, color)) = cube.split_once(" ") {
                    match color.trim() {
                        "red" => game.r_cubes = std::cmp::max(game.r_cubes, count.trim().parse::<i32>().unwrap()),
                        "green" => game.g_cubes = std::cmp::max(game.g_cubes, count.trim().parse::<i32>().unwrap()),
                        "blue" => game.b_cubes = std::cmp::max(game.b_cubes, count.trim().parse::<i32>().unwrap()),
                        _ => panic!("Invalid color"),
                    }
                }
            }
        }

        game
    }
}

struct Game {
    game_id: i32,
    r_cubes: i32,
    g_cubes: i32,
    b_cubes: i32,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.r_cubes <= 12 && self.g_cubes <= 13 && self.b_cubes <= 14
    }

    fn power(&self) -> i32 {
        self.r_cubes * self.g_cubes * self.b_cubes
    }
}
