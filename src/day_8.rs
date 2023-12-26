const INPUTS_FOLDER: &str = "src/inputs/day_8";

#[warn(dead_code)]
use crate::generic;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
}


#[derive(Debug, Eq, PartialEq, Clone)]
struct Map {
    instructions: Vec<char>,
    index: usize,
}

impl Map {
    fn from_string(input_line: &String) -> Self {
        return Self {
            instructions: input_line.chars().collect(),
            index: 0,
        }
    }

    fn get_direction(&mut self) -> Direction {
        let next_instruction: Direction;
        match self.instructions[self.index] {
            'L' => next_instruction = Direction::Left,
            'R' => next_instruction = Direction::Right,
            _ => next_instruction = Direction::Left,
        }

        self.index += 1;
        self.index %= self.instructions.len();

        return next_instruction;
    }
}


#[derive(Debug, Eq, PartialEq, Clone)]
struct NetworkNode {
    start: String,
    left: String,
    right: String,
}

impl NetworkNode {
    fn from_string(input_line: &String) -> Self {
        return Self {
            start: input_line[..3].to_string(),
            left: input_line[7..10].to_string(),
            right: input_line[12..15].to_string(),
        }
    }

    fn get_next_node(&self, direction: Direction) -> String {
        match direction {
            Direction::Left => self.left.clone(),
            Direction::Right => self.right.clone(),
        }
    }

    fn is_finished(&self) -> bool {
        match self.start.chars().nth(2).expect("Node start is too small") {
            'Z' => true,
            _ => false,
        }
    }
}

fn all_true(input_bool: Vec<bool>) -> bool {
    let bool_set: HashSet<bool> = HashSet::from_iter(input_bool.clone());
    match bool_set.len() {
        1 => input_bool[0],
        _ => false,
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut map: Map = Map::from_string(&input_lines[0]);
    let nodes: Vec<NetworkNode> = input_lines[2..].iter().map(|x| NetworkNode::from_string(x)).collect();
    let nodes_hash: HashMap<String, NetworkNode> = nodes.iter().map(|x| (x.start.clone(), x.clone())).collect::<HashMap<String, NetworkNode>>();

    let mut start_nodes: Vec<&NetworkNode> = Vec::new();
    if part_2 {
        for n in nodes.iter() {
            if n.start.chars().nth(2).expect("Node start too short") == 'A' {
                start_nodes.push(n);
            }
        }
    } else {
        start_nodes = vec![nodes_hash.get("AAA").expect("AAA does not exist in hashset")];
    }

    let mut all_steps: Vec<usize> = Vec::new();
    println!("Start Nodes = {:?}", start_nodes);

    for i in 0..start_nodes.len() {
        let mut current_node = start_nodes[i];
        let mut steps: usize = 0;

        while !current_node.is_finished() {
            current_node = &nodes_hash[&current_node.get_next_node(map.get_direction())];
            steps += 1;
        }

        all_steps.push(steps);
    }

    let mut lcm = 1;
    for step in all_steps {
        lcm = get_lowest_common_multiple(lcm, step);
    }

    return lcm;
}

fn get_lowest_common_multiple(a: usize, b: usize) -> usize {
    let mut x: usize;
    let mut y: usize;

    if a > b {
        x = a;
        y = b;
    } else {
        x = b;
        y = a;
    }
    
    let mut remainder = x % y;

    while remainder != 0 {
        x = y;
        y = remainder;
        remainder = x % y;
    }

    return (a * b)/y;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
        // Do a quick test here
        println!("{}", get_lowest_common_multiple(5, 3));
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2);
    }

    #[test]
    fn example_2_part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 6);
    }
    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 15989);
    }

    #[test]
    fn example_1_part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_3.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 6);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 13830919117339);
    }
}