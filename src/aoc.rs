pub mod day1;

fn str_reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}
