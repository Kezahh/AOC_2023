#[warn(dead_code)]
use crate::generic;
use std::{collections::HashMap, thread::current};

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

#[derive(Clone, Debug, PartialEq)]
struct SeedRange {
    start: usize,
    length: usize,
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

    fn src_range_to_dst(&self, src_range: &SeedRange) -> Vec<SeedRange> {
        let mut out_ranges: Vec<SeedRange> = Vec::new();
        let mut current_ranges: Vec<SeedRange> = vec![src_range.clone()];

        println!("Doing src_range_to_dst");
        println!("\tseed_ranges = {:?}", current_ranges);
        println!("\tMap ranges = {:?}", &self.ranges);

        for range in &self.ranges {
            let mut new_current_ranges: Vec<SeedRange> = Vec::new();

            for seed_range in current_ranges {
                println!("\t\tseed_range = {:?}", seed_range);
                println!("\t\trange = {:?}", range);
                if seed_range.start < range.src_start {
                    if (seed_range.start + seed_range.length) < range.src_start {
                        // No overlap.
                        new_current_ranges.push(seed_range);
                    } else {
                        // Some overlap
                        let start_range: SeedRange = SeedRange{
                            start: seed_range.start,
                            length: range.src_start - seed_range.start
                        };
                        new_current_ranges.push(start_range);

                        if (range.src_start + range.length) > (seed_range.start + seed_range.length) {
                            // end in the middle of the range
                            let mid_range: SeedRange = SeedRange {
                                start: range.dest_start,
                                length: (seed_range.start + seed_range.length) - range.src_start
                            };
                            out_ranges.push(mid_range);
                        } else {
                            // seed range ends after
                            let mid_range: SeedRange = SeedRange {
                                start: range.dest_start,
                                length: range.length
                            };
                            let end_range: SeedRange = SeedRange {
                                start: range.src_start + range.length,
                                length: (seed_range.start + seed_range.length) - (range.src_start + range.length)
                            };

                            out_ranges.push(mid_range);
                            new_current_ranges.push(end_range);
                        }
                    }
                } else if seed_range.start < (range.src_start + range.length) {
                    // seed range starts in the middle of range
                    if (seed_range.start + seed_range.length) <= (range.src_start + range.length) {
                        // finished inside the range
                        let mid_range: SeedRange = SeedRange {
                            start: range.dest_start + (seed_range.start - range.src_start),
                            length: seed_range.length
                        };
                        out_ranges.push(mid_range);
                    } else {
                        // finishes outside the range
                        let mid_range: SeedRange = SeedRange {
                            start: range.dest_start + (seed_range.start - range.src_start),
                            length: (range.src_start + range.length) - seed_range.start
                        };
                        let end_range: SeedRange = SeedRange {
                            start: range.src_start + range.length,
                            length: (seed_range.start + seed_range.length) - (range.src_start + range.length)
                        };

                        out_ranges.push(mid_range);
                        new_current_ranges.push(end_range);
                    }
                } else {
                    // seed range starts after the range has finished
                    new_current_ranges.push(seed_range);
                }
            }

            current_ranges = new_current_ranges.clone();
        }

        // Any ranges not covered are just turned into the dest as is.
        out_ranges.append(&mut current_ranges);
        
        out_ranges.sort_by_key(|x| x.start);
        println!("\tout_ranges = {:?}", out_ranges);

        let in_length: usize = src_range.length;
        let out_length: usize = out_ranges.iter().map(|x| x.length).sum();

        println!("\tin_length = {}, out_length = {}", in_length, out_length);
        assert_eq!(in_length, out_length);
        

        return out_ranges;
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
    
    let mut seed_ranges: Vec<SeedRange> = Vec::new();
    for i in (0..almanac.seeds.len()).step_by(2) {
        seed_ranges.push(SeedRange { start: almanac.seeds[i], length: almanac.seeds[i+1] })
    }


    let mut all_locations: Vec<usize> = Vec::new();

    if !part_2 {
        // PART 1
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
    } else {
        // PART 2
        let mut current_ranges: Vec<SeedRange>;
        let mut current_category: Category = Category::seed;

        while almanac.maps_by_src.contains_key(&current_category) {
            current_ranges = seed_ranges.clone();
            let mut new_ranges: Vec<SeedRange> = Vec::new();

            println!("current_ranges {:?}", current_ranges);
            

            for seed_range in current_ranges {
                new_ranges.append(&mut almanac.maps_by_src[&current_category].src_range_to_dst(&seed_range));
            }

            println!("new_ranges {:?}", new_ranges);
            current_category = almanac.maps_by_src[&current_category].dst_category.clone();
            seed_ranges = new_ranges.clone();
        }

        println!("{:?}", seed_ranges);
        println!("end category = {:?}", current_category);

        return seed_ranges.iter().map(|x| x.start).min().expect("oh no not a number");
    }
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
        assert!(answer == 46);
    }

    #[test]
    fn run_cases() {
        let test_ranges: Vec<Range> = vec![Range{src_start: 20, dest_start: 40, length: 20}];
        let test_map = Map {
            src_category: Category::seed,
            dst_category: Category::soil,
            ranges: test_ranges
        };

        // 1 Start before. End before
        let seed_range: SeedRange = SeedRange { start: 0, length: 5 };
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(seed_range, out_ranges[0]);

        // 2 Start before. End middle
        let seed_range: SeedRange = SeedRange { start: 0, length: 25 };
        let expected_ranges: Vec<SeedRange> = vec![
            SeedRange{start: 0, length: 20}, SeedRange{start: 40, length: 5}
        ];
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(out_ranges, expected_ranges);
        
        // 3 Start before. End after
        let seed_range: SeedRange = SeedRange { start: 0, length: 50 };
        let expected_ranges: Vec<SeedRange> = vec![
            SeedRange { start: 0, length: 20 },
            SeedRange { start: 40, length: 20 },
            SeedRange { start: 40, length: 10 }
        ];
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(out_ranges, expected_ranges);

        // 4 Start middle. End middle
        let seed_range: SeedRange = SeedRange { start: 25, length: 10 };
        let expected_ranges: Vec<SeedRange> = vec![
            SeedRange { start: 45, length: 10 }
        ];
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(out_ranges, expected_ranges);

        // 5 Start middle. End after
        let seed_range: SeedRange = SeedRange { start: 25, length: 25 };
        let expected_ranges: Vec<SeedRange> = vec![
            SeedRange { start: 40, length: 10 },
            SeedRange { start: 45, length: 15 }
        ];
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(out_ranges, expected_ranges);

        // 6 Start after. End after
        let seed_range: SeedRange = SeedRange { start: 45, length: 5 };
        let expected_ranges: Vec<SeedRange> = vec![
            SeedRange { start: 45, length: 5 }
        ];
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(out_ranges, expected_ranges);

        // 7 Start exactly. End middle
        let seed_range: SeedRange = SeedRange { start: 20, length: 10 };
        let expected_ranges: Vec<SeedRange> = vec![
            SeedRange { start: 40, length: 10 }
        ];
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(out_ranges, expected_ranges);

        // 8 Start exactly. End exactly
        let seed_range: SeedRange = SeedRange { start: 20, length: 20 };
        let expected_ranges: Vec<SeedRange> = vec![
            SeedRange { start: 40, length: 20 }
        ];
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(out_ranges, expected_ranges);

        // 8 Start middle. End exactly
        let seed_range: SeedRange = SeedRange { start: 30, length: 10 };
        let expected_ranges: Vec<SeedRange> = vec![
            SeedRange { start: 50, length: 10 }
        ];
        let out_ranges: Vec<SeedRange> = test_map.src_range_to_dst(&seed_range);
        assert_eq!(out_ranges, expected_ranges);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle("src/inputs/day_5/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 104070862);

        //9284340 too low
    }
}