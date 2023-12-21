use regex::Regex;

pub struct Day5;

impl crate::aoc::Compute for Day5 {
    fn compute_part_one(&self, version: String) -> String {
        let almanac = self.input_load("1".to_string(), version.clone());
        almanac.seeds.iter().map(|s| almanac.process_map_path(*s, "seed".to_string(), 0).0).min().unwrap().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let almanac = self.input_load("2".to_string(), version.clone());
        almanac.seeds_ranges().iter().map(|r| almanac.process_map_path_range(r)).min().unwrap().to_string()
    }
}

impl Day5 {
    fn input_load(&self, part: String, version: String) -> Almanac {
        let input = crate::aoc::input_load("5".to_string(), part, version.clone());
        let regex = Regex::new(r"(?<dst>\d+)\s+(?<src>\d+)\s+(?<len>\d+)").unwrap();

        let mut almanac = Almanac { seeds: Vec::new(), seed_to_soil_map: Map { converters: Vec::new() }, soil_to_fertilizer_map: Map { converters: Vec::new() }, fertilizer_to_water_map: Map { converters: Vec::new() }, water_to_light_map: Map { converters: Vec::new() }, light_to_temperature_map: Map { converters: Vec::new() }, temperature_to_humidity_map: Map { converters: Vec::new() }, humidity_to_location_map: Map { converters: Vec::new() } };
        let mut current_map: String = "".to_string();
        let mut converters = Vec::new();
        
        for line in input.lines() {
            if line.contains("seeds:") {
                let parts = line.split(":");
                almanac.seeds = parts.last().unwrap().split(" ").filter(|&s| !s.is_empty()).map(|n| n.trim().parse::<usize>().unwrap()).collect();
            }

            if line.contains(" map:") {
                current_map = line.split(" ").next().unwrap().to_string().replace("-", "_");
                converters.clear();
            }

            if let Some(caps) = regex.captures(line) {
                converters.push(Converter {
                    src: Range::create(caps["src"].parse::<usize>().unwrap(), caps["len"].parse::<usize>().unwrap()),
                    dst: Range::create(caps["dst"].parse::<usize>().unwrap(), caps["len"].parse::<usize>().unwrap())
                })
            }

            if line.is_empty() && !current_map.is_empty() {
                almanac.set_map_converters(current_map.to_string(), converters.clone());
            }
        }

        if !current_map.is_empty() {
            almanac.set_map_converters(current_map.to_string(), converters.clone());
        }

        almanac
    }
}

#[derive(Debug, Clone)]
struct Range {
    min: usize,
    max: usize
}

impl Range {
    fn create(min: usize, len: usize) -> Self {
        Self { min: min, max: min + len - 1 }
    }
}

#[derive(Debug, Clone)]
struct Converter {
    src: Range,
    dst: Range
}

#[derive(Debug)]
struct Map {
    converters: Vec<Converter>
}

impl Map {
    fn convert_src_to_dst(&self, src: usize) -> (usize, usize) {
        let mut dst = src;
        let mut end = src;

        for converter in self.converters_without_gaps().iter() {
            if src >= converter.src.min && src <= converter.src.max {
                dst = converter.dst.min + (src - converter.src.min);
                end = converter.src.max - src;
                break;
            }
        }

        (dst, end)
    }

    fn converters_sorted_by_src(&self) -> Vec<Converter> {
        let mut converters = self.converters.clone();
        converters.sort_by(|a, b| a.src.min.cmp(&b.src.min));
        converters
    }

    fn converters_without_gaps(&self) -> Vec<Converter> {
        let mut converters = Vec::new();
        let mut last = 0 as usize;

        for converter in self.converters_sorted_by_src().iter() {
            if converter.src.min > last {
                converters.push(Converter {
                    src: Range::create(last, converter.src.min),
                    dst: Range::create(last, converter.src.min)
                });
            }

            converters.push(converter.clone());
            last = converter.src.max + 1;
        }

        converters
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map
}

impl Almanac {
    fn process_map_path(&self, value: usize, item: String, mut flow: usize) -> (usize, usize) {
        let (curr, skip) = match item.as_str() {
            "seed" => self.seed_to_soil_map.convert_src_to_dst(value),
            "soil" => self.soil_to_fertilizer_map.convert_src_to_dst(value),
            "fertilizer" => self.fertilizer_to_water_map.convert_src_to_dst(value),
            "water" => self.water_to_light_map.convert_src_to_dst(value),
            "light" => self.light_to_temperature_map.convert_src_to_dst(value),
            "temperature" => self.temperature_to_humidity_map.convert_src_to_dst(value),
            "humidity" => self.humidity_to_location_map.convert_src_to_dst(value),
            _ => (value, flow)
        };

        flow = *Vec::from(&[flow, skip]).iter().min().unwrap();

        match item.as_str() {
            "seed" => self.process_map_path(curr, "soil".to_string(), flow),
            "soil" => self.process_map_path(curr, "fertilizer".to_string(), flow),
            "fertilizer" => self.process_map_path(curr, "water".to_string(), flow),
            "water" => self.process_map_path(curr, "light".to_string(), flow),
            "light" => self.process_map_path(curr, "temperature".to_string(), flow),
            "temperature" => self.process_map_path(curr, "humidity".to_string(), flow),
            "humidity" => self.process_map_path(curr, "location".to_string(), flow),
            _ => (curr, flow)
        }
    }

    fn process_map_path_range(&self, seed_range: &Range) -> usize {
        let mut cur = seed_range.min;
        let mut max = seed_range.max;
        let mut val = Vec::new();

        while cur <= max {
            let (loc, skp) = self.process_map_path(cur, "seed".to_string(), usize::MAX);
            cur += skp + 1;
            val.push(loc);
        }

        *val.iter().min().unwrap()
    }

    fn seeds_ranges(&self) -> Vec<Range> {
        let mut ranges = Vec::new();

        for pair in self.seeds.chunks_exact(2) {
            ranges.push(Range::create(pair[0], pair[1]));
        }

        ranges
    }

    fn set_map_converters(&mut self, map: String, converters: Vec<Converter>) {
        match map.as_str() {
            "seed_to_soil" => self.seed_to_soil_map.converters = converters,
            "soil_to_fertilizer" => self.soil_to_fertilizer_map.converters = converters,
            "fertilizer_to_water" => self.fertilizer_to_water_map.converters = converters,
            "water_to_light" => self.water_to_light_map.converters = converters,
            "light_to_temperature" => self.light_to_temperature_map.converters = converters,
            "temperature_to_humidity" => self.temperature_to_humidity_map.converters = converters,
            "humidity_to_location" => self.humidity_to_location_map.converters = converters,
            _ => panic!("Invalid map name"),
        }
    }
}
