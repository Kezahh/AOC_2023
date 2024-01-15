const INPUTS_FOLDER: &str = "src/inputs/day_15";

use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::generic;

#[derive(Clone, Eq, PartialEq, Hash)]
enum Operation {
    Dash,
    Equals,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let return_char = match &self {
            Self::Dash => '-',
            Self::Equals => '=',
        };
        write!(f, "{}", return_char)
    } 
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '-' => Self::Dash,
            '=' => Self::Equals,
            _ => panic!("Bad character given")
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Step {
    operation_type: Operation,
    label: String,
    focal_length: usize,
    hash_value: usize,
}

impl From<&String> for Step {
    fn from(value: &String) -> Self {
        let operation_index: usize;
        let focal_length: usize;
        if value.contains('-') {
            operation_index = value.find('-').unwrap();
            focal_length = 0;
        } else if value.contains('=') {
            operation_index = value.find('=').unwrap();
            focal_length = value[(operation_index + 1)..].parse::<usize>().expect("Focal length is not a number!");
        } else {
            panic!("Instruction does not have operation {}", value);
        }

        return Self {
            operation_type: Operation::from(value.chars().collect::<Vec<char>>()[operation_index]),
            label: value[..operation_index].to_string(),
            focal_length: focal_length,
            hash_value: hash_algorithm(&value[..operation_index].to_string()),
        };
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}={}]", self.label, self.focal_length)
    }
}

#[derive(Clone)]
struct LightBox {
    step_labels: HashMap<String, Step>,
    step_order: HashMap<Step, usize>,
    step_max_index: usize,
}

impl LightBox {
    fn new() -> Self {
        return Self {
            step_labels: HashMap::new(),
            step_order: HashMap::new(),
            step_max_index: 0,
        };
    }

    fn add_step(&mut self, step: &Step) {
        match step.operation_type {
            Operation::Dash => {
                if let Some(removed_step) = self.step_labels.remove(&step.label) {
                    self.step_order.remove(&removed_step);
                }
            }
            Operation::Equals => {
                if let Some(current_step) = self.step_labels.get(&step.label) {
                    let current_index = self.step_order.remove(&current_step).expect("Current step missing from step order!!");
                    
                    self.step_order.insert(step.clone(), current_index);
                    self.step_labels.insert(step.label.clone(), step.clone());
                } else {
                    self.step_max_index += 1;
                    self.step_order.insert(step.clone(), self.step_max_index);

                    self.step_labels.insert(step.label.clone(), step.clone());
                }
            }
        }
    }

    fn get_product_value(&self) -> usize {
        let mut box_steps: Vec<Step> = self.step_order.keys().map(|x| x.clone()).collect::<Vec<Step>>();
        box_steps.sort_by_key(|x| self.step_order[x]);

        return (0..box_steps.len()).map(|i| (i+1) * box_steps[i].focal_length).sum();
    }
}

fn hash_algorithm(input: &String) -> usize {
    let mut result: usize = 0;
    for c in input.chars() {
        result += c as usize;
        result *= 17;
        result %= 256;
    }

    return result;
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let steps: Vec<String> = input_lines.first().expect("Input lines is empty!").split(',').map(|x| x.to_string()).collect::<Vec<String>>();
    let real_steps: Vec<Step> = steps.iter().map(|x| Step::from(x)).collect::<Vec<Step>>();

    let mut all_boxes: Vec<LightBox> = vec![LightBox::new(); 256];

    for step in real_steps {
        all_boxes[step.hash_value].add_step(&step);
    }

    for (i, b) in all_boxes.iter().enumerate() {
        if b.step_labels.len() != 0 {
            let mut box_steps: Vec<Step> = b.step_order.keys().map(|x| x.clone()).collect::<Vec<Step>>();
            box_steps.sort_by_key(|x| b.step_order[x]);
            let steps_string: String = box_steps.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(" ");
            println!("Box {}: {}", i, steps_string);
        }
    }

    if !part_2 {
        return steps.iter().map(|x| hash_algorithm(x)).sum();
    } else {
        return (0..all_boxes.len()).map(|i| (i+1) * all_boxes[i].get_product_value()).sum();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        assert_eq!(hash_algorithm(&"rn=1".to_string()), 30);
        assert_eq!(hash_algorithm(&"rn".to_string()), 0);
        assert_eq!(hash_algorithm(&"cm".to_string()), 0);
        assert_eq!(hash_algorithm(&"qp".to_string()), 1);
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1320);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 519041);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 145);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 260530);
    }
}