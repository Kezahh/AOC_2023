const INPUTS_FOLDER: &str = "src/inputs/day_12";

#[warn(dead_code)]
use crate::generic;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl SpringState {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => Self::Unknown,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::Operational => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SpringRecord {
    spring_map: Vec<usize>,
    spring_mask: Vec<char>,
}

impl SpringRecord {
    fn from_line(input_line: &String) -> Self {
        let spring_mask: Vec<char> = input_line[0..input_line.find(' ').expect("No space in input line")].chars().collect::<Vec<char>>();
        let spring_map: Vec<usize> = input_line[(input_line.find(' ').expect("No space in input line")+1)..].split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        return Self {
            spring_map: spring_map,
            spring_mask: spring_mask,
        };
    }

    fn expand_spring(mut self) -> Self {
        // used for part 2.  
        self.spring_mask = vec![String::from_iter(self.spring_mask); 5].join("?").chars().collect::<Vec<char>>();
        self.spring_map = vec![self.spring_map; 5].concat();

        return self;
    }

    fn get_mask_length(&self) -> usize {
        return self.spring_mask.len();
    }

    fn get_min_length(&self) -> usize {
        return self.spring_map.iter().sum::<usize>() + self.spring_map.len() - 1;
    }

    fn get_spares(&self) -> usize {
        return self.get_mask_length() - self.get_min_length();
    }

    fn gaps_count(&self) -> usize {
        return self.spring_map.len() + 1;
    }

    fn _get_gap_set(&self) -> HashSet<Vec<usize>> {
        let mut all_gaps: Vec<Vec<usize>> = Vec::new();
        let mut gaps: Vec<usize>;
        let mut spares_index: Vec<usize> = vec![0; self.get_spares()];
        let total_loops: usize = (self.gaps_count() as i32).pow(self.get_spares() as u32) as usize;

        for x in 1..(total_loops + 1) {
            gaps = vec![0; self.gaps_count()];
            
            for i in spares_index.iter() {
                gaps[*i] += 1;
            }

            if x > 0 {
                for y in 0..spares_index.len() {
                    if x % ((self.gaps_count() as i32).pow(y as u32)) as usize == 0 {
                        spares_index[y] += 1;
                        if spares_index[y] == gaps.len() {
                            spares_index[y] = 0;
                        }
                    }
                }
            } else {
                spares_index[0] += 1;
            }

            //println!("\t{:?} x = {}", gaps, x);
            all_gaps.push(gaps.clone());
        }

        let gaps_set: HashSet<Vec<usize>> = HashSet::from_iter(all_gaps.iter().map(|x| x.clone()));

        return gaps_set;
    }

    fn get_possibilities(&self, possibility_cache: &mut HashMap<SpringRecord, usize>) -> usize {
        /* 
         * Algorithm logic from manhunto
         * https://github.com/manhunto/advent-of-code-rs/blob/master/src/solutions/day12.rs
         * 
         */


        // println!("\tGetting possibilities for {:?} {:?}", self.spring_mask, self.spring_map);


        // Check if answer already exists in cache.
        if let Some(result) = possibility_cache.get(self) {
            // println!("\t\talready exists {:?} {:?}", self.spring_mask, self.spring_map);
            // println!("\t\t\tpossibilities = {}", result);
            return *result;
        }
        

        // check if map is empty.
        if self.spring_map.len() == 0 {
            let result: usize;

            // if the mask contains any damaged springs then it is not possible as the 
            // map is empty. No more damanged springs.
            if self.spring_mask.contains(&SpringState::Damaged.as_char()) {
                result = 0;
            } else {
                result = 1;
            }

            possibility_cache.insert(self.clone(), result);

            // println!("\t\tspring map is empty {:?} {:?}", self.spring_mask, self.spring_map);
            // println!("\t\t\tpossibilities = {}", result);

            return result;
        }


        // Check if there are enough springs left for group.
        if self.spring_mask.len() < self.get_min_length() {
            possibility_cache.insert(self.clone(), 0);

            // println!("\t\tSpring mask is too small {:?} {:?}", self.spring_mask, self.spring_map);
            // println!("\t\t\tpossibilities = 0");

            return 0;
        }


        // Check if first spring is operational.
        // If it is, then skip ahead one character in the mask and same spring map.
        let first_spring = SpringState::from_char(self.spring_mask[0]);
        if first_spring == SpringState::Operational {
            let new_spring_record = Self {
                spring_map: self.spring_map.clone(),
                spring_mask: self.spring_mask[1..].to_vec(),
            };

            let result = new_spring_record.get_possibilities(possibility_cache);
            possibility_cache.insert(self.clone(), result);

            // println!("\t\tfirst is operational - skip ahead {:?} {:?}", self.spring_mask, self.spring_map);
            // println!("\t\t\tpossibilities = {}", result);

            return result;
        }


        let spring_count = self.spring_map[0];
        let all_broken_or_wild = self.spring_mask[..spring_count].iter().all(|s| *s != SpringState::Operational.as_char());
        let end_of_springs: usize;

        if self.spring_mask.len() < (spring_count + 1) {
            end_of_springs = self.spring_mask.len();
        } else {
            end_of_springs = spring_count + 1;
        }


        let mut possibilities: usize = 0;

        // If all the springs in the current count are Broken or Wild
        // and the first spring after is not-broken, then add possibilities.
        if all_broken_or_wild {
            if (self.spring_mask.len() > spring_count && SpringState::from_char(self.spring_mask[spring_count]) != SpringState::Damaged)
                    || self.spring_mask.len() <= spring_count {
                let new_spring_record = Self {
                    spring_map: self.spring_map[1..].to_vec(),
                    spring_mask: self.spring_mask[end_of_springs..].to_vec(),
                };

                possibilities += new_spring_record.get_possibilities(possibility_cache);
            }
        }

        if first_spring == SpringState::Unknown {
            let new_spring_record = Self {
                spring_map: self.spring_map.clone(),
                spring_mask: self.spring_mask[1..].to_vec(),
            };

            possibilities += new_spring_record.get_possibilities(possibility_cache);
        }

        possibility_cache.insert(self.clone(), possibilities);

        // println!("\t\tFinished function {:?} {:?}", self.spring_mask, self.spring_map);
        // println!("\t\t\tpossibilities = {}", possibilities);

        return possibilities;
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut spring_records = input_lines.iter().map(|x| SpringRecord::from_line(x)).collect::<Vec<SpringRecord>>();

    if part_2 {
        spring_records = spring_records.iter().map(|x| x.clone().expand_spring()).collect::<Vec<SpringRecord>>();
    }

    let mut total_sum: usize = 0;
    let mut index: usize = 0;

    println!("started\n");

    let mut possibility_cache: HashMap<SpringRecord, usize> = HashMap::new();
    for s in spring_records {
        //println!("gaps = {}, spare = {}", s.spring_map.len() + 1, s.get_spares());
        //println!("Spring set = {:?}", s.spring_map);
        //println!("Spring mask = {:?}", s.spring_mask);
        print!("\rdoing {}/{}", index + 1, input_lines.len());
        total_sum += s.get_possibilities(&mut possibility_cache);
        index += 1;
    }
    println!("\nDone");



    return total_sum;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn check_spring(input_line: &str, answer: usize, part_2: bool) {
        let input_lines: Vec<String> = vec![input_line.to_string()];
        let mut spring_records = input_lines.iter().map(|x| SpringRecord::from_line(x)).collect::<Vec<SpringRecord>>();
        if part_2 {
            spring_records = spring_records.iter().map(|x| x.clone().expand_spring()).collect::<Vec<SpringRecord>>();
        }

        let target_record = spring_records.first().expect("Spring records is empty??");
        let mut possibility_cache: HashMap<SpringRecord, usize> = HashMap::new();

        println!("gaps = {}, spare = {}", target_record.spring_map.len() + 1, target_record.get_spares());
        println!("Spring set = {:?}", target_record.spring_map);
        println!("Spring mask = {:?}", target_record.spring_mask);

        assert_eq!(target_record.get_possibilities(&mut possibility_cache), answer);
    }

    #[test]
    fn very_quick_test() {
        let cheeky_vec = vec!['a', 'b', 'c', 'd'];
        let target_char: char = 'b';

        assert!(cheeky_vec.contains(&target_char));
    }


    #[test]
    fn quick_test() {
        // Do a quick test here
        check_spring("???.### 1,1,3", 1, false);
        check_spring(".??..??...?##. 1,1,3", 4, false);
        check_spring("?#?#?#?#?#?#?#? 1,3,1,6", 1, false);
        check_spring("????.#...#... 4,1,1", 1, false);
        check_spring("????.######..#####. 1,6,5", 4, false);
        check_spring("?###???????? 3,2,1", 10, false);
    }

    #[test]
    fn quick_untest2() {
        // Do a quick test here
        check_spring("???.### 1,1,3", 1, true);
        check_spring(".??..??...?##. 1,1,3", 16384, true);
        check_spring("?#?#?#?#?#?#?#? 1,3,1,6", 1, true);
        check_spring("????.#...#... 4,1,1", 16, true);
        check_spring("????.######..#####. 1,6,5", 2500, true);
        check_spring("?###???????? 3,2,1", 506250, true);
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 21);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 6827);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 525152);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1537505634471);
    }
}