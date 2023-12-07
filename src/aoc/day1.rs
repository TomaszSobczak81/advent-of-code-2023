use regex::Regex;

pub struct Day1;

impl crate::aoc::Compute for Day1 {
    fn compute_part_one(&self, version: String) -> String {
        let input = crate::aoc::input_load("1".to_string(), "1".to_string(), version.clone());

        return self.compute(input);
    }

    fn compute_part_two(&self, version: String) -> String {
        let forward_pattern = self.forward_mapping().keys().map(|s| s.to_string()).collect::<Vec<_>>().join("|");
        let reverse_pattern = self.reverse_mapping().keys().map(|s| s.to_string()).collect::<Vec<_>>().join("|");

        let numeric_regex = Regex::new(r"(\d)").unwrap();
        let forward_regex = Regex::new(&forward_pattern).unwrap();
        let reverse_regex = Regex::new(&reverse_pattern).unwrap();

        let input = crate::aoc::input_load("1".to_string(), "2".to_string(), version.clone());
        let lines: Vec<_> = input.lines().map(|line| self.normalize_input_line(line, &numeric_regex, &forward_regex, &reverse_regex)).collect();

        return self.compute(lines.join("\n"));
    }
}

impl Day1 {
    fn compute(&self, input: String) -> String {
        let mut result: i32 = 0;
        let re = Regex::new(r"(\d)").unwrap();

        for line in input.lines() {
            let enil = crate::aoc::str_reverse(line);
            let f = re.find(line).unwrap().as_str();
            let l = re.find(enil.as_str()).unwrap().as_str();

            result += format!("{}{}", f, l).parse::<i32>().unwrap();
        }

        return result.to_string();
    }

    fn forward_mapping(&self) -> std::collections::HashMap<&'static str, &'static str> {
        std::collections::HashMap::from([
            ("zero", "0"),
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9")
        ])
    }

    fn reverse_mapping(&self) -> std::collections::HashMap<&'static str, &'static str> {
        std::collections::HashMap::from([
            ("orez", "0"),
            ("eno", "1"),
            ("owt", "2"),
            ("eerht", "3"),
            ("ruof", "4"),
            ("evif", "5"),
            ("xis", "6"),
            ("neves", "7"),
            ("thgie", "8"),
            ("enin", "9")
        ])
    }

    fn normalize_input_line(&self, line: &str, numeric_regex: &regex::Regex, forward_regex: &regex::Regex, reverse_regex: &regex::Regex) -> String {
        let mut result = line.to_string();
        let mut finding = forward_regex.find(&result);

        if let Some(f) = finding {
            let number = numeric_regex.find(&result);
            if number.is_none() || number.unwrap().start() > f.start() {
                result = forward_regex.replace(&result, &**self.forward_mapping().get(f.as_str()).unwrap()).to_string();
            }
        }

        result = crate::aoc::str_reverse(&result); // reverse the line to find on the other side
        finding = reverse_regex.find(&result);

        if let Some(f) = finding {
            let number = numeric_regex.find(&result);
            if number.is_none() || number.unwrap().start() > f.start() {
                result = reverse_regex.replace(&result, &**self.reverse_mapping().get(f.as_str()).unwrap()).to_string();
            }
        }

        result = crate::aoc::str_reverse(&result); // back to initial order
        result
    }
}
