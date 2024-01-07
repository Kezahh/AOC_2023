const INPUTS_FOLDER: &str = "src/inputs/day_11";

#[warn(dead_code)]
use crate::generic;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum DataType {
    EmptySpace,
    Galaxy,
}

impl DataType {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Galaxy,
            _ => Self::EmptySpace,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::EmptySpace => '.',
            Self::Galaxy => '#',
        }
    }
}

#[derive(Debug, Eq,PartialEq, Clone)]
struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn distance(&self, other: &Self) -> usize {
        let horizontal_distance = self.col.abs_diff(other.col);
        let vertical_distance = self.row.abs_diff(other.row);

        return horizontal_distance + vertical_distance;
    }
}

#[derive(Debug)]
struct GalaxyMap {
    actual_map: Vec<Vec<DataType>>,
    galaxies: Vec<Galaxy>,
}

impl GalaxyMap {
    fn from_input_lines(input_lines: &Vec<String>, empty_space_add: usize) -> Self {
        let first_map = input_lines.iter().map(|x| x.chars().map(|x| DataType::from_char(x)).collect::<Vec<DataType>>()).collect::<Vec<Vec<DataType>>>();
        let rows_blank = first_map.iter().map(|x| HashSet::<&DataType>::from_iter(x).len() == 1).collect::<Vec<bool>>();
        let mut cols_blank: Vec<bool> = Vec::new();
        
        for col in 0..first_map[0].len() {
            cols_blank.push(HashSet::<DataType>::from_iter(first_map.iter().map(|x| x[col].clone())).len() == 1);
        }

        let mut galaxies: Vec<Galaxy> = Vec::new();
        let mut row_count: usize = 0;
        
        for row in 0..first_map.len() {
            let mut col_count: usize = 0;
            for col in 0..first_map[0].len() {
                if first_map[row][col] == DataType::Galaxy {
                    galaxies.push(Galaxy { row: row_count, col: col_count })
                }
                if cols_blank[col] {
                    col_count += empty_space_add;
                }
                col_count += 1;
            }
            
            if rows_blank[row] {
                row_count += empty_space_add;
            }
            row_count += 1;
        }
        

        return Self {
            actual_map: first_map,
            galaxies: galaxies,
        }
    }

    fn print_map(&self) {
        for row in 0..self.actual_map.len() {
            let mut row_string = String::new();
            for col in 0..self.actual_map[0].len() {
                row_string.push(self.actual_map[row][col].as_char());
            }
            println!("{}", row_string);
        }
    }
}


fn solve_puzzle(input_filename: String, _part_2: bool, empty_space_add: usize) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let galaxy_map: GalaxyMap = GalaxyMap::from_input_lines(&input_lines, empty_space_add);

    galaxy_map.print_map();

    let mut all_distances: usize = 0;

    for i in 0..galaxy_map.galaxies.len() {
        for j in (i+1)..galaxy_map.galaxies.len() {
            all_distances += galaxy_map.galaxies[i].distance(&galaxy_map.galaxies[j]);
        }
    }

    return all_distances;
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
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false, 1);
        println!("Answer = {:?}", answer);
        assert!(answer == 374);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false, 1);
        println!("Answer = {:?}", answer);
        assert!(answer == 10885634);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true, 9);
        println!("Answer = {:?}", answer);
        assert!(answer == 1030);
    }

    #[test]
    fn example_3() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true, 99);
        println!("Answer = {:?}", answer);
        assert!(answer == 8410);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true, 999999);
        println!("Answer = {:?}", answer);
        assert!(answer == 707505470642);
    }
}