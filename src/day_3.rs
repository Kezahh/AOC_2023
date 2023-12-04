#[warn(dead_code)]
use crate::generic;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Number {
    value: usize,
    row: usize,
    left: usize,
    right: usize,
}

#[derive(Debug, Clone)]
struct Gear {
    row: usize,
    col: usize,
    numbers: Vec<Number>,
}

fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);

    //let mut games: Vec<HashMap<String, usize>> = Vec::new();

    let char_map: Vec<Vec<char>> = get_char_map(&input_lines);
    let all_numbers: Vec<Number> = get_all_numbers(&char_map);
    let part_numbers: Vec<Number> = all_numbers.iter().cloned().filter(|x| has_symbol_next_to_number(&char_map, &*x)).collect();
    let mut asterisks: Vec<Gear> = get_all_asterisks(&char_map);

    let mut all_numbers_by_row: Vec<Vec<Number>> = Vec::new();
    for row in 0..char_map.len() {
        all_numbers_by_row.push(all_numbers.iter().cloned().filter(|x| x.row == row).collect());
    }

    for asterisk in asterisks.iter_mut() {
        if asterisk.row > 0 {
            asterisk.numbers.append(&mut all_numbers_by_row[asterisk.row - 1].iter().cloned().filter(|x| is_number_neighbour(x, asterisk.row - 1, asterisk.col)).collect());
        }
        asterisk.numbers.append(&mut all_numbers_by_row[asterisk.row].iter().cloned().filter(|x| is_number_neighbour(x, asterisk.row, asterisk.col)).collect());
        if asterisk.row < all_numbers_by_row.len() {
            asterisk.numbers.append(&mut all_numbers_by_row[asterisk.row + 1].iter().cloned().filter(|x| is_number_neighbour(x, asterisk.row + 1, asterisk.col)).collect());
        }
    }

    println!("{:?}", asterisks);

    if !part_2 {
        return part_numbers.iter().map(|x| x.value).sum();
    } else {
        return asterisks.iter().filter(|x| x.numbers.len() == 2).map(|x| x.numbers[0].value * x.numbers[1].value).sum();
    }
}

fn get_char_map(input_lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut char_map: Vec<Vec<char>> = Vec::new();

    char_map.push(vec!['.'; input_lines[0].len()+2]);
    for line in input_lines {
        let mut new_line: Vec<char> = Vec::new();
        new_line.push('.');
        new_line.append(&mut line.chars().collect());
        new_line.push('.');
        char_map.push(new_line);
    }
    char_map.push(vec!['.'; input_lines[0].len()+2]);

    return char_map;
}

fn get_all_numbers(char_map: &Vec<Vec<char>>) -> Vec<Number> {
    let mut all_numbers: Vec<Number> = Vec::new();

    for row in 0..char_map.len() {
        let mut number_string: String = String::new();
        let mut left = 0;
        for col in 0..char_map[0].len() {
            if char_map[row][col].is_numeric() {
                if number_string.len() == 0 {
                    // We are at the start of a number.
                    left = col;
                }
                number_string.push(char_map[row][col]);
            } else {
                // did we just finish a number?
                if number_string.len() > 0 {
                    let new_number = Number{
                        value: number_string.parse::<usize>().unwrap(),
                        row: row,
                        left: left,
                        right: col - 1
                    };
                    all_numbers.push(new_number);
                    number_string = "".to_string();
                }
            }
        }
    }

    return all_numbers;
}

fn get_all_asterisks(char_map: &Vec<Vec<char>>) -> Vec<Gear> {
    let mut all_asterisks: Vec<Gear> = Vec::new();
    for row in 0..char_map.len() {
        for col in 0..char_map[0].len() {
            if char_map[row][col] == '*' {
                all_asterisks.push(Gear { row: row, col: col, numbers: Vec::new() })
            }
        }
    }

    return all_asterisks;
}

fn is_symbol(input_char: char) -> bool {
    // assume it's a symbol if its not numeric and not a '.'
    return !input_char.is_numeric() && input_char != '.';
}

fn has_symbol_next_to_number(char_map: &Vec<Vec<char>>, target_number: &Number) -> bool {
    let mut has_symbol = false;
    //println!("Checking from Rows {} to {}", target_number.row - 1, target_number.row + 2);
    for row in (target_number.row - 1)..(target_number.row + 2) {
        //println!("Checking from Cols {} to {}", target_number.left - 1, target_number.right + 2);
        for col in (target_number.left - 1)..(target_number.right + 2) {
            has_symbol = has_symbol || is_symbol(char_map[row][col]);
        }
    }

    return has_symbol;
}

fn is_number_neighbour(target_number: &Number, row: usize, col: usize) -> bool {
    return (target_number.row >= (row - 1)) && (target_number.row <= (row + 1))
        && (target_number.left <= col + 1) && (target_number.right >= col - 1);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        assert!(is_symbol('.') == false);
        assert!(is_symbol('0') == false);
        assert!(is_symbol('9') == false);
        assert!(is_symbol('#') == true);
        assert!(is_symbol('%') == true);
        assert!(is_symbol('/') == true);
        
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle("src/inputs/day_3/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 4361);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle("src/inputs/day_3/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 556367);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle("src/inputs/day_3/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 467835);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle("src/inputs/day_3/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 89471771);
    }
}