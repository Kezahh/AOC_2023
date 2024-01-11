const INPUTS_FOLDER: &str = "src/inputs/day_14";

use std::{fmt::Display, collections::HashMap};

use crate::generic;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Rock {
    Round,
    Cube,
    Ground,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => Self::Ground,
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_char = match self {
            Self::Round => 'O',
            Self::Cube => '#',
            Self::Ground => '.',
        };
        write!(f, "{}", display_char)
    }
}

struct RockWrapper(Vec<Rock>);

impl Display for RockWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|x| format!("{}", x)).collect::<String>())
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Platform {
    rows: Vec<Vec<Rock>>,
}

impl From<Vec<String>> for Platform {
    fn from(value: Vec<String>) -> Self {
        Self {
            rows: value.iter().map(|row| row.chars().map(|x| Rock::from(x)).collect::<Vec<Rock>>()).collect::<Vec<Vec<Rock>>>(),
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_string = self.rows
            .iter()
            .map(|x| x.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(""))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", display_string)
    }
}

impl Platform {
    fn get_north_load(&self) -> usize {
        let cols = (0..self.rows[0].len()).map(|i| self.rows.iter().map(|row| row[i].clone()).collect::<Vec<Rock>>()).collect::<Vec<Vec<Rock>>>();
        let max_level: usize = cols[0].len();
        let mut total_count: usize = 0;

        for col in cols.iter() {
            total_count += (0..col.len()).filter(|i| col[*i] == Rock::Round).map(|i| col.len() - i).sum::<usize>();
        }

        return total_count;
    }

    fn tilt_platform(&mut self, direction: Direction, cache: &mut HashMap<Vec<Rock>, Vec<Rock>>) {
        match direction {
            Direction::North => {
                for col_index in 0..self.rows[0].len() {
                    let new_vec = Self::tilt_vec(&self.rows.iter().map(|x| x[col_index].clone()).collect::<Vec<Rock>>(), cache);
                    for row_index in 0..self.rows.len() {
                        self.rows[row_index][col_index] = new_vec[row_index].clone();
                    }
                }
            },
            Direction::West => {
                for row_index in 0..self.rows.len() {
                    let new_vec = Self::tilt_vec(&self.rows[row_index], cache);
                    self.rows[row_index] = new_vec;
                }
            },
            Direction::South => {
                // South is bottom to top. Reverse the input to tilt and reverse the output.
                for col_index in 0..self.rows[0].len() {
                    let new_vec = Self::tilt_vec(&self.rows.iter().rev().map(|x| x[col_index].clone()).collect::<Vec<Rock>>(), cache)
                        .into_iter().rev().collect::<Vec<Rock>>();
                    for row_index in 0..self.rows.len() {
                        self.rows[row_index][col_index] = new_vec[row_index].clone();
                    }
                }
            },
            Direction::East => {
                // East rolls the rocks to the right. Reverse the input to tilt and reverse the output.
                for row_index in 0..self.rows.len() {
                    let new_vec = Self::tilt_vec(&self.rows[row_index].clone().into_iter().rev().collect::<Vec<Rock>>(), cache)
                        .into_iter().rev().collect::<Vec<Rock>>();
                    self.rows[row_index] = new_vec;
                }
            },
        }
    }

    fn tilt_vec(input_vec: &Vec<Rock>, cache: &mut HashMap<Vec<Rock>, Vec<Rock>>) -> Vec<Rock> {
        if let Some(result) = cache.get(input_vec) {
            return result.clone();
        }

        let mut current_level: usize = 0;
        let mut new_vec: Vec<Rock> = vec![Rock::Ground; input_vec.len()];

        for i in 0..input_vec.len() {
            if input_vec[i] == Rock::Round {
                new_vec[current_level] = Rock::Round;
                current_level += 1;
            } else if input_vec[i] == Rock::Cube {
                new_vec[i] = Rock::Cube;
                current_level = i + 1;
            }
        }

        cache.insert(input_vec.clone(), new_vec.clone());

        return new_vec;
    }

    fn spin_platform(&mut self, cache: &mut HashMap<Vec<Rock>, Vec<Rock>>) {
        for d in vec![Direction::North, Direction::West, Direction::South, Direction::East] {
            self.tilt_platform(d, cache);
        }
    }

    fn spin_cycle(&self, cycles: usize) -> Self {
        let mut cache: HashMap<Vec<Rock>, Vec<Rock>> = HashMap::new();
        let mut platform_cache: HashMap<Self, Self> = HashMap::new();
        let mut platform_index: HashMap<Self, usize> = HashMap::new();
        let mut platform_list: Vec<Self> = Vec::new();
        let mut found_repeat: bool = false;
        let mut start_index: usize = 0;
        let mut platform_loop: Vec<usize> = Vec::new();
        let mut loop_start: usize = 0;


        println!("Running Spins.");
        println!("");

        let mut current_platform: Self = self.clone();
        for i in 0..cycles {
            if let Some(spun_platform) = platform_cache.get(&current_platform) {
                println!("Platform {} comes from platform {}", i, platform_index[&current_platform]);
                
                current_platform = spun_platform.clone();
                if !found_repeat {
                    found_repeat = true;
                    loop_start = i;
                    start_index = platform_index[&current_platform];

                    let mut current_index = start_index;
                    platform_loop.push(current_index);
                    current_platform = platform_cache.get(&current_platform).expect("Platform not in HashMap!").clone();
                    current_index = platform_index[&current_platform];
                    while current_index != start_index {
                        platform_loop.push(current_index);
                        current_platform = platform_cache.get(&current_platform).expect("Platform not in HashMap!").clone();
                        current_index = platform_index[&current_platform];
                    }
                    break;
                }
            } else {
                let pre_spin_platform = current_platform.clone();

                current_platform.spin_platform(&mut cache);

                // println!("");
                // println!("Prespin");
                // println!("{}", pre_spin_platform);
                // println!("");
                // println!("After spin cycles");
                // println!("{}", current_platform);

                platform_cache.insert(pre_spin_platform.clone(), current_platform.clone());
                platform_index.insert(pre_spin_platform.clone(), i);
                platform_list.push(pre_spin_platform);
            }

            // println!("");
            // println!("After {} cycles", i+1);
            // println!("{}", current_platform);
        }

        if found_repeat {
            println!("{:?}", platform_loop);
            for x in 0..platform_list.len() {
                println!("Platform {} = {}", x, platform_list[x].get_north_load());
            }
            return platform_list[platform_loop[(cycles - loop_start - 1) % platform_loop.len()]].clone();
        }

        return current_platform;
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut platform = Platform::from(input_lines);

    // println!("{}", platform);

    if part_2 {
        platform = platform.spin_cycle(1000000000);
    } else {
        let mut cache: HashMap<Vec<Rock>, Vec<Rock>> = HashMap::new();
        platform.tilt_platform(Direction::North, &mut cache);
    }

    return platform.get_north_load();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        println!("");
        for i in 0..1000000000 {
            if i/1000 % 10000 == 0 {
                print!("\rwe at: {}", i);
            }
        }
    }

    #[test]
    fn test_tilt() {
        let my_vec = "..O.O..#O..O..O#...O..#O.O.#".chars().map(|x| Rock::from(x)).collect::<Vec<Rock>>();
        let mut cache: HashMap<Vec<Rock>, Vec<Rock>> = HashMap::new();
        let new_vec = Platform::tilt_vec(&my_vec, &mut cache);

        println!("old {}", RockWrapper(my_vec));
        println!("new {}", RockWrapper(new_vec));
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 136);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 108826);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 64);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 99291);
    }
}