const INPUTS_FOLDER: &str = "src/inputs/day_12";

#[warn(dead_code)]
use crate::generic;
use std::collections::HashSet;

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

#[derive(Debug, Clone)]
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

    fn get_gap_set(&self) -> HashSet<Vec<usize>> {
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

    fn get_possibilities2(&self) -> usize {
        let gaps_set: HashSet<Vec<usize>> = self.get_gap_set();
        let mut possibilities: HashSet<Vec<char>> = HashSet::new();

        for gaps in gaps_set {
            //println!("\tgaps = {:?}", gaps);
            let mut possibility: Vec<char> = Vec::new();
            for i in 0..self.spring_map.len() {
                if i == 0 {
                    possibility.append(&mut vec![SpringState::Operational.as_char(); gaps[i]]);
                } else {
                    // gaps in between get an extra 1 as they always need to be a gap.
                    // Only the first and last gap can be 0.
                    possibility.append(&mut vec![SpringState::Operational.as_char(); gaps[i] + 1]);
                }
                possibility.append(&mut vec![SpringState::Damaged.as_char(); self.spring_map[i]]);
            }
            possibility.append(&mut vec![SpringState::Operational.as_char(); *gaps.last().expect("Gaps is empty")]);

            let mut possibility_with_mask = possibility.clone();
            for i in 0..self.spring_mask.len() {
                if self.spring_mask[i] == SpringState::Unknown.as_char() {
                    possibility_with_mask[i] = SpringState::Unknown.as_char();
                }
            }
            //println!("\tpossibility = {:?}", possibility);

            if possibility_with_mask == self.spring_mask {
                //println!("\tpossibility = {:?}", possibility);
                possibilities.insert(possibility);
                //println!("\t\tyep");
            }
        }
        

        return possibilities.len();
    }

    fn get_possibilities(&self) -> usize {
        let mut wild_places: Vec<usize> = Vec::new();
        for i in 0..self.spring_mask.len() {
            if self.spring_mask[i] == '?' {
                wild_places.push(i);
            }
        }
        // let mut wilds_on_off: Vec<bool> = vec![false; wild_places.len()];

        // let mut all_possibilities: Vec<Vec<usize>> = Vec::new();
        // let total_loops: usize = (2 as i32).pow(wilds_on_off.len() as u32) as usize;

        // for x in 0..total_loops {
        //     if x > 0 {
        //         for y in 0..wilds_on_off.len() {
        //             if x % ((2 as i32).pow(y as u32)) as usize == 0 {
        //                 wilds_on_off[y] = !wilds_on_off[y];
        //             }
        //         }
        //     }

        //     //println!("\t{:?} x = {}", gaps, x);
        //     all_possibilities.push(wilds_on_off.iter().map(|x| *x as usize).collect::<Vec<usize>>());
        // }

        let total_loops: usize = (2 as i32).pow(wild_places.len() as u32) as usize;
        let mut spring_possibilities: Vec<String> = Vec::new();

        for x in 0..total_loops {
            let mut possibility: Vec<char> = self.spring_mask.clone();
            for i in 0..wild_places.len() {
                if (x >> i) & 1 == 1 {
                    possibility[wild_places[i]] = SpringState::Damaged.as_char();
                } else {
                    possibility[wild_places[i]] = SpringState::Operational.as_char();
                }
            }
            let possibility_string: String = String::from_iter(possibility.iter());
            //println!("\t{:?}", possibility_string);
            let spring_map = possibility_string.split(SpringState::Operational.as_char())
                .filter(|x| x.len() != 0)
                .map(|x| x.len())
                .collect::<Vec<usize>>();

            //println!("\t\t\t{:?}", spring_map);
            if spring_map == self.spring_map {
                spring_possibilities.push(possibility_string);
                //println!("\t{:?}", possibility);
            }
            
        }

        return spring_possibilities.len();
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
    for s in spring_records {
        //println!("gaps = {}, spare = {}", s.spring_map.len() + 1, s.get_spares());
        //println!("Spring set = {:?}", s.spring_map);
        //println!("Spring mask = {:?}", s.spring_mask);
        print!("\rdoing {}/{}", index + 1, input_lines.len());
        total_sum += s.get_possibilities();
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

        println!("gaps = {}, spare = {}", target_record.spring_map.len() + 1, target_record.get_spares());
        println!("Spring set = {:?}", target_record.spring_map);
        println!("Spring mask = {:?}", target_record.spring_mask);

        assert_eq!(target_record.get_possibilities(), answer);
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
        assert!(answer == 30);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 7185540);
    }
}