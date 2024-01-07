const INPUTS_FOLDER: &str = "src/inputs/day_9";

#[warn(dead_code)]
use crate::generic;
use std::collections::HashSet;

#[derive(Debug,Eq,PartialEq, Clone)]
struct History {
    sequence: Vec<i32>,
    gradients: Vec<Vec<i32>>,
}

impl History {
    fn from_string(input_string: &String) -> Self {
        return Self {
            sequence: input_string.split(" ").map(|x| x.parse::<i32>().expect("Input is not a number")).collect::<Vec<i32>>(),
            gradients: Vec::new(),
        }
    }

    fn calculate_gradients(mut self) -> Self {
        let mut current_sequence = self.sequence.clone();
        let mut sequence_set: HashSet<i32> = HashSet::from_iter(current_sequence.clone());

        while !(sequence_set.len() == 1 && sequence_set.get(&0) != None) {
            let mut gradients: Vec<i32> = Vec::new();
            for i in 0..(current_sequence.len() - 1) {
                gradients.push(current_sequence[i+1] - current_sequence[i]);
            }
            current_sequence = gradients.clone();
            sequence_set = HashSet::from_iter(current_sequence.clone());
            self.gradients.push(gradients);
        }

        return self;
    }

    fn extrapolate_next_number(&self) -> i32 {
        let mut next_number: i32 = 0;
        for i in (0..self.gradients.len() - 1).rev() {
            next_number = self.gradients[i].last().expect("Gradient vector empty").clone() + next_number;
        }

        return self.sequence.last().expect("sequence is empty") + next_number;
    }

    fn extrapolate_previous_number(&self) -> i32 {
        let mut previous_number: i32 = 0;
        for i in (0..self.gradients.len() - 1).rev() {
            //println!("\tlast gradient = {}", next_number);
            previous_number = self.gradients[i].first().expect("Gradient vector empty").clone() - previous_number;
            //println!("\t\tnext number = {}", next_number);
        }

        return self.sequence.first().expect("sequence is empty") - previous_number;
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut histories: Vec<History> = input_lines.iter().map(|x| History::from_string(x)).collect::<Vec<History>>();

    histories = histories.iter().map(|x| x.clone().calculate_gradients()).collect::<Vec<History>>();

    let mut extrapolated_numbers: Vec<i32> = Vec::new();
    for h in histories {
        if !part_2 {
            extrapolated_numbers.push(h.extrapolate_next_number());
        } else {
            extrapolated_numbers.push(h.extrapolate_previous_number());
        }
    }

    return extrapolated_numbers.iter().sum::<i32>() as usize;
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
        assert!(answer == 114);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1995001648);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 2);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 988);
    }
}