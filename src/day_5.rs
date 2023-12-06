#[warn(dead_code)]
use crate::generic;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Category {
    seed,
    soil,
    fertilizer,
    water,
    light,
    temperature,
    humidity,
    location,
}

impl Category {
    fn from_string(input_string: &str) -> Self {
        return match input_string {
            "seed" => Self::seed,
            "soil" => Self::soil,
            "fertilizer" => Self::fertilizer,
            "water" => Self::water,
            "light" => Self::light,
            "temperature" => Self::temperature,
            "humidity" => Self::humidity,
            "location" => Self::location,
            _ => Self::seed,
        }
    }
}

#[derive(Clone, Debug)]
struct Range {
    src_start: usize,
    dest_start: usize,
    length: usize,
}

impl Range {
    fn from_string(input_string: &str) -> Self {
        let inputs: Vec<usize> = input_string.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        return Self { src_start: inputs[1], dest_start: inputs[0], length: inputs[2] };
    }
}

#[derive(Clone, Debug)]
struct Map {
    src_category: Category,
    dst_category: Category,
    ranges: Vec<Range>,
}

impl Map {
    fn src_to_dst(&self, src_number: usize) -> usize {
        //println!("Checking src_number {}", src_number);
        for range in &self.ranges {
            //println!("\t{:?}", range);
            if src_number >= range.src_start && src_number < (range.src_start + range.length) {
                return range.dest_start + (src_number - range.src_start);
            }
        }

        return src_number;
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps_by_src: HashMap<Category, Map>,
}

impl Almanac {
    fn from_lines(input_lines: &Vec<String>) -> Self {
        let seeds: Vec<usize> = input_lines[0][(input_lines[0].find(":").unwrap() + 1)..].trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        let mut all_maps: HashMap<Category, Map> = HashMap::new();

        let mut line_index = 2;
        while line_index < input_lines.len() {
            let mut current_line: String = input_lines[line_index].clone();
            let src_string: String = current_line[..current_line.find("-").unwrap()].to_string();
            let dst_string: String = current_line[(current_line.find("-to-").unwrap()+4)..current_line.find(" ").unwrap()].to_string();
            let mut new_map = Map{
                src_category: Category::from_string(src_string.as_str()),
                dst_category: Category::from_string(dst_string.as_str()),
                ranges: Vec::new(),
            };

            // Get all ranges
            line_index += 1;
            current_line = input_lines[line_index].clone();
            while current_line != "" {
                new_map.ranges.push(Range::from_string(&current_line));
                line_index += 1;
                if line_index == input_lines.len() {
                    break;
                }
                current_line = input_lines[line_index].clone();
            }

            all_maps.insert(new_map.src_category.clone(), new_map);
            line_index += 1;
        }


        return Self {
            seeds: seeds,
            maps_by_src: all_maps,
        }
    }
}




fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let almanac: Almanac = Almanac::from_lines(&input_lines);

    let mut all_locations: Vec<usize> = Vec::new();
    for seed in almanac.seeds {
        let mut current_category: Category = Category::seed;
        let mut current_number: usize = seed;
        let mut indent: String = "".to_string();
        while almanac.maps_by_src.contains_key(&current_category) {
            //println!("{}{:?} {:?}", indent, current_category, current_number);
            current_number = almanac.maps_by_src[&current_category].src_to_dst(current_number);
            current_category = almanac.maps_by_src[&current_category].dst_category.clone();
            indent.push('\t');
        }
        //println!("{}{:?} {:?}", indent, current_category, current_number);
        //println!("Seed {:?} becomes {:?} {:?}", seed, almanac.maps_by_src[&Category::seed].src_to_dst(seed), almanac.maps_by_src[&Category::seed].dst_category)

        all_locations.push(current_number);
    }

    return *all_locations.iter().min().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle("src/inputs/day_5/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 35);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle("src/inputs/day_5/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 324724204);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle("src/inputs/day_5/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 30);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle("src/inputs/day_5/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 7185540);
    }
}