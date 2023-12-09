use regex::Regex;

pub struct Day5;

impl crate::aoc::Compute for Day5 {
    fn compute_part_one(&self, version: String) -> String {
        let almanac = self.input_load("1".to_string(), version.clone());
        almanac.seeds.iter().map(|s| almanac.get_location_by_seed(*s)).min().unwrap().to_string()
    }

    fn compute_part_two(&self, version: String) -> String {
        let almanac = self.input_load("2".to_string(), version.clone());
        // almanac.seeds_ranges().iter().map(|s| almanac.get_location_by_seed(*s)).min().unwrap().to_string()

        // for pair in self.seeds.chunks_exact(2) {
        //     ranges.push(Range::create(pair[0], pair[1]));
        //     seeds.append(&mut (pair[0]..pair[0] + pair[1]).collect());
        // }

        for c in almanac.seeds.chunks_exact(2) {
            let location_s = almanac.get_location_by_seed(c[0]);
            let location_t = almanac.get_location_by_seed(c[0] + c[1] - 1);
            println!("{:?} {:?}", c[0], location_s);
            println!("{:?} {:?}", c[0] + c[1] - 1, location_t);
            println!("------------------");
        }

        almanac.seeds_ranges().iter().map(|s| almanac.get_location_by_seed(*s)).min().unwrap().to_string()
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
    fn convert_src_to_dst(&self, src: usize) -> usize {
        let mut dst = src;

        for converter in self.converters.iter() {
            if src >= converter.src.min && src <= converter.src.max {
                dst = converter.dst.min + (src - converter.src.min);
                break;
            }
        }

        dst
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
    fn get_location_by_seed(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil_map.convert_src_to_dst(seed);
        let fertilizer = self.soil_to_fertilizer_map.convert_src_to_dst(soil);
        let water = self.fertilizer_to_water_map.convert_src_to_dst(fertilizer);
        let light = self.water_to_light_map.convert_src_to_dst(water);
        let temperature = self.light_to_temperature_map.convert_src_to_dst(light);
        let humidity = self.temperature_to_humidity_map.convert_src_to_dst(temperature);
        let location = self.humidity_to_location_map.convert_src_to_dst(humidity);

        location
    }

    fn seeds_ranges(&self) -> Vec<usize> {
        let mut ranges = Vec::new();
        let mut seeds = Vec::new();

        for pair in self.seeds.chunks_exact(2) {
            ranges.push(Range::create(pair[0], pair[1]));
            seeds.append(&mut (pair[0]..pair[0] + pair[1]).collect());
        }

        // println!("{:?} {:?}", ranges, 1);

        seeds
        // [79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67].iter().map(|s| *s as usize).collect()
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
