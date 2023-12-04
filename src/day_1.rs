#[warn(dead_code)]
use crate::generic;

#[derive(Debug, Clone)]
struct NumberWord {
    number: String,
    number_real: usize,
    index: usize,
}

fn solve_part_one(input_lines: Vec<String>) -> usize {
    let mut input_numbers: Vec<Vec<usize>> = Vec::new();

    for line in input_lines {
        let mut current_numbers: Vec<usize> = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                current_numbers.push((c as usize) - 48);  //48 is ascii for '0'
            }
        }
        //println!("{:?}", current_numbers);
        input_numbers.push(current_numbers);
    }

    let mut answer_numbers: Vec<usize> = Vec::new();
    for line in input_numbers {
        answer_numbers.push((line[0]*10) + line[line.len()-1]);
    }

    return answer_numbers.iter().sum();
}

fn solve_part_two(mut input_lines: Vec<String>) -> usize {
    let input_lines_original = input_lines.clone();
    let number_words: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    
    for i in 0..input_lines.len() {
        let mut finished = false;
        print!("{:?}", input_lines[i]);

        while !finished {
            let mut indexes: Vec<NumberWord> = Vec::new();
            for j in 0..number_words.len() {
                let index = input_lines[i].find(number_words[j]);
                if index != None {
                    indexes.push(NumberWord{number: number_words[j].to_string(), number_real: j, index: index.unwrap()});
                }
            }
            //println!("{:?}", input_lines[i]);
            //println!("{:?}", indexes);
            if indexes.len() > 0 {
                indexes.sort_by(|a, b| a.index.cmp(&b.index));

                let number_word = indexes[0].clone();

                input_lines[i] = input_lines[i][..number_word.index].to_string() + &number_word.number_real.to_string() +
                    &input_lines[i][(number_word.index + number_word.number.len())..].to_string();
            } else {
                finished = true;
            }
        }
        println!(" --> {:?}", input_lines[i]);
        //input_lines[i] = input_lines[i].replace(&number_word.number, number_word.number_real.to_string().as_str());
        
        //input_lines[i] = input_lines[i].replace(number_words[j], j.to_string().as_str());

        //println!("{:?}", indexes);
    }


    let mut input_numbers: Vec<Vec<usize>> = Vec::new();

    for (i, line) in input_lines.iter().enumerate() {
        let mut current_numbers: Vec<usize> = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                current_numbers.push((c as usize) - 48);  //48 is ascii for '0'
            }
        }
        //println!("{:?}", current_numbers);

        println!("{:?} --> {:?} --> {:?}, {:?}", input_lines_original[i], input_lines[i], &current_numbers[0], &current_numbers[current_numbers.len() - 1]);
        input_numbers.push(current_numbers);

        
    }

    let mut answer_numbers: Vec<usize> = Vec::new();
    for line in input_numbers {
        answer_numbers.push((line[0]*10) + line[line.len()-1]);
    }

    return answer_numbers.iter().sum();
}

fn solve_part_twob(mut input_lines: Vec<String>) -> usize {
    let input_lines_original = input_lines.clone();
    let number_words: Vec<Vec<&str>> = vec![
        vec!["zero", "0o"],
        vec!["one", "o1e"],
        vec!["two", "t2o"],
        vec!["three", "t3e"],
        vec!["four", "4"],
        vec!["five", "5e"],
        vec!["six", "6"],
        vec!["seven", "7n"],
        vec!["eight", "e8t"],
        vec!["nine", "n9e"]
    ];
    
    for i in 0..input_lines.len() {
        print!("{:?}", input_lines[i]);

        for number_word in &number_words {
            input_lines[i] = input_lines[i].replace(number_word[0], number_word[1]);
        }
        println!("{:?}", input_lines[i]);
    }


    let mut input_numbers: Vec<Vec<usize>> = Vec::new();

    for (i, line) in input_lines.iter().enumerate() {
        let mut current_numbers: Vec<usize> = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                current_numbers.push((c as usize) - 48);  //48 is ascii for '0'
            }
        }
        //println!("{:?}", current_numbers);

        println!("{:?} --> {:?} --> {:?}, {:?}", input_lines_original[i], input_lines[i], &current_numbers[0], &current_numbers[current_numbers.len() - 1]);
        input_numbers.push(current_numbers);

        
    }

    let mut answer_numbers: Vec<usize> = Vec::new();
    for line in input_numbers {
        answer_numbers.push((line[0]*10) + line[line.len()-1]);
    }

    return answer_numbers.iter().sum();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input_lines = generic::read_in_file("src/inputs/day_1/input_example_1.txt");
        assert!(solve_part_one(input_lines) == 142);
    }

    #[test]
    fn part_1() {
        let input_lines = generic::read_in_file("src/inputs/day_1/input.txt");
        let answer = solve_part_one(input_lines);
        println!("Answer = {}", answer);
        assert!(answer == 54331);
    }

    #[test]
    fn example_2() {
        let input_lines = generic::read_in_file("src/inputs/day_1/input_example_2.txt");
        let answer = solve_part_twob(input_lines);
        println!("Answer = {}", answer);
        assert!(answer == 281);
    }

    #[test]
    fn part_2() {
        let input_lines = generic::read_in_file("src/inputs/day_1/input.txt");
        let answer = solve_part_twob(input_lines);
        println!("Answer = {}", answer);
        assert!(answer == 54518);
    }
}