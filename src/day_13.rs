const INPUTS_FOLDER: &str = "src/inputs/day_13";

use std::fmt::Display;

use crate::generic;

#[derive(Clone, Eq, PartialEq, Hash)]
enum Terrain {
    Ash,
    Rock,
}

impl From<char> for Terrain {
    fn from(input_char: char) -> Self {
        match input_char {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Bad input type"),
        }
    }
}

impl Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Ash => '.',
            Self::Rock => '#',
        };
        
        return write!(f, "{}", c);
    }
}


struct Pattern {
    rows: Vec<Vec<Terrain>>,
    cols: Vec<Vec<Terrain>>,
}

impl From<Vec<String>> for Pattern {
    fn from(input_lines: Vec<String>) -> Self {
        let rows = input_lines.iter().map(|x| x.chars().map(|x| Terrain::from(x)).collect::<Vec<Terrain>>()).collect::<Vec<Vec<Terrain>>>();
        let cols = (0..rows[0].len()).map(|i| rows.iter().map(|row| row[i].clone()).collect::<Vec<Terrain>>()).collect::<Vec<Vec<Terrain>>>();

        return Self { rows: rows, cols: cols };
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_string = self.rows
            .iter()
            .map(|x| x.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(""))
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", display_string)
    }
}

impl Pattern {
    fn get_mirror(map_lines: &Vec<Vec<Terrain>>) -> Option<usize> {
        // map_lines can be either rows or cols in the map.

        let mut last_line: &Vec<Terrain> = map_lines.first().expect("Map lines input is empty!");
        let mut line_index: usize = 1;
        while line_index < map_lines.len() {
            if &map_lines[line_index] == last_line {
                let short_side_length: usize = line_index.min(map_lines.len() - line_index);
                let sides_match: bool = (0..short_side_length).all(|i| map_lines[line_index-1-i] == map_lines[line_index + i]);
                if sides_match {
                    return Some(line_index);
                }
            }
            last_line = &map_lines[line_index];
            line_index += 1;
        }

        return None;
    }

    fn get_left_columns(&self) -> Option<usize> {
        return Self::get_mirror(&self.cols);
    }

    fn get_top_rows(&self) -> Option<usize> {
        return Self::get_mirror(&self.rows);
    }

    fn get_mirror_with_smude(map_lines: &Vec<Vec<Terrain>>) -> Option<usize> {
        // map_lines can be either rows or cols in the map.

        let mut last_line: &Vec<Terrain> = map_lines.first().expect("Map lines input is empty!");
        let mut line_index: usize = 1;
        let mut check_all: bool = false;
        let mut smudge_fixed: bool = false;

        while line_index < map_lines.len() {
            if &map_lines[line_index] == last_line {
                check_all = true;
            } else if !smudge_fixed && almost_equal(&last_line, &map_lines[line_index]) {
                check_all = true;
                smudge_fixed = true;
            }

            if check_all {
                let short_side_length: usize = line_index.min(map_lines.len() - line_index);
                let mut sides_match: bool = true;
                for i in 1..short_side_length {
                    if map_lines[line_index-1-i] != map_lines[line_index + i] {
                        if !smudge_fixed && almost_equal(&map_lines[line_index-1-i], &map_lines[line_index+i]) {
                            smudge_fixed = true;
                        } else {
                            sides_match = false;
                            break;
                        }
                    }
                }

                if sides_match && smudge_fixed {
                    return Some(line_index);
                }
            }

            smudge_fixed = false;
            check_all = false;
            last_line = &map_lines[line_index];
            line_index += 1;
        }

        return None;
    }

    fn get_left_columns_with_smudge(&self) -> Option<usize> {
        return Self::get_mirror_with_smude(&self.cols);
    }

    fn get_top_rows_with_smudge(&self) -> Option<usize> {
        return Self::get_mirror_with_smude(&self.rows);
    }
}

fn almost_equal(first: &Vec<Terrain>, second: &Vec<Terrain>) -> bool {
    // println!("Checking {}", first.iter().map(|x| format!("{}", x)).collect::<String>());
    // println!("\tagainst {}", second.iter().map(|x| format!("{}", x)).collect::<String>());
    // println!("\tresult = {}", (0..first.len()).map(|i| (first[i] != second[i]) as usize).sum::<usize>());
    return (0..first.len()).map(|i| (first[i] != second[i]) as usize).sum::<usize>() == 1;
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut index: usize = 0;
    let mut start_index: usize = 0;
    let mut patterns: Vec<Pattern> = Vec::new();
    while index < input_lines.len() {
        if input_lines[index] == "" {
            patterns.push(Pattern::from(input_lines[start_index..index].to_vec()));
            start_index = index + 1;
        }
        index += 1;
    }
    patterns.push(Pattern::from(input_lines[start_index..].to_vec()));

    let mut result: usize = 0;
    if !part_2 {
        result += patterns.iter().map(|x| x.get_left_columns()).filter(|x| x.is_some()).map(|x| x.unwrap()).sum::<usize>();
        result += patterns.iter().map(|x| x.get_top_rows()).filter(|x| x.is_some()).map(|x| x.unwrap() * 100).sum::<usize>();
    } else {
        result += patterns.iter().map(|x| x.get_left_columns_with_smudge()).filter(|x| x.is_some()).map(|x| x.unwrap()).sum::<usize>();
        result += patterns.iter().map(|x| x.get_top_rows_with_smudge()).filter(|x| x.is_some()).map(|x| x.unwrap() * 100).sum::<usize>();
    }

    // for p in patterns {
    //     if let Some(left_cols) = p.get_left_columns_with_smudge() {
    //         println!("Left is {}", left_cols);
    //     }
    //     if let Some(top_rows) = p.get_top_rows_with_smudge() {
    //         println!("Top is {}", top_rows);
    //     }
    // }

    return result;
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
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 405);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 30705);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 400);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 44615);
    }
}