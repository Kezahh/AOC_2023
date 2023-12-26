const INPUTS_FOLDER: &str = "src/inputs/day_10";

#[warn(dead_code)]
use crate::generic;




#[derive(Debug, Eq, PartialEq, Clone)]
struct Position {
    row: usize,
    col: usize,
}


#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum PipeType {
    StartPosition,
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
}

impl PipeType {
    fn from_char(input_char: char) -> Self {
        match input_char {
            'S' => Self::StartPosition,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthToEast,
            'J' => Self::NorthToWest,
            'F' => Self::SouthToEast,
            '7' => Self::SouthToWest,
            _ => Self::Ground,
        }
    }

    fn connects_direction(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Up => match self {
                Self::StartPosition => true,
                Self::Vertical => true,
                Self::NorthToEast => true,
                Self::NorthToWest => true,
                _ => false,
            }
            Direction::Down => match self {
                Self::StartPosition => true,
                Self::Vertical => true,
                Self::SouthToEast => true,
                Self::SouthToWest => true,
                _ => false,
            }
            Direction::Left => match self {
                Self::StartPosition => true,
                Self::Horizontal => true,
                Self::NorthToWest => true,
                Self::SouthToWest => true,
                _ => false,
            }
            Direction::Right => match self {
                Self::StartPosition => true,
                Self::Horizontal => true,
                Self::NorthToEast => true,
                Self::SouthToEast => true,
                _ => false,
            }
        }
    }

    fn connects(&self, other: &Self, other_direction: &Direction) -> bool {
        return self.connects_direction(&other_direction) && other.connects_direction(&other_direction.opposite());
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Pipe {
    pipe_type: PipeType,
    position: Position,
}

struct PipeMap {
    actual_map: Vec<Vec<Pipe>>,
}

impl PipeMap {
    fn from_input_lines(input_lines: &Vec<String>) -> Self {
        let mut map: Vec<Vec<Pipe>> = Vec::new();
        
        for row in 0..input_lines.len() {
            let mut pipe_row: Vec<Pipe> = Vec::new();
            for col in 0..input_lines[row].len() {
                pipe_row.push(Pipe{
                    pipe_type: PipeType::from_char(input_lines[row].chars().nth(col).expect("Not a char")),
                    position: Position { row: row, col: col },
                })
            }
            map.push(pipe_row);
        }

        return Self {
            actual_map: map,
        };
    }

    fn get_pipe(&self, row: usize, col: usize) -> &Pipe {
        return &self.actual_map[row][col];
    }

    fn get_start_pipe(&self) -> Option<&Pipe> {
        for pipe_row in self.actual_map.iter() {
            for pipe in pipe_row {
                if pipe.pipe_type == PipeType::StartPosition {
                    return Some(&pipe);
                }
            }
        }
    
        return None;
    }

    fn get_neighbour_pipe_direction(&self, target_pipe: &Pipe, direction: &Direction) -> Option<&Pipe> {
        match direction {
            Direction::Up =>
                if target_pipe.position.row > 0 {
                    return Some(&self.actual_map[target_pipe.position.row - 1][target_pipe.position.col]);
                },
            Direction::Down =>
                if target_pipe.position.row < (self.actual_map.len() - 1) {
                    return Some(&self.actual_map[target_pipe.position.row + 1][target_pipe.position.col]);
                },
            Direction::Left =>
                if target_pipe.position.col > 0 {
                    return Some(&self.actual_map[target_pipe.position.row][target_pipe.position.col - 1]);
                },
            Direction::Right =>
                if target_pipe.position.col < (self.actual_map[0].len() - 1) {
                    return Some(&self.actual_map[target_pipe.position.row][target_pipe.position.col + 1]);
                },
        }

        return None
    }

    fn get_neighbour_pipes(&self, target_pipe: &Pipe) -> Vec<&Pipe> {
        let mut neighbour_pipes: Vec<&Pipe> = Vec::new();
        let directions: Vec<Direction> = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        //println!("Get neighbours of: {:?}", target_pipe);
        for d in directions {
            if target_pipe.pipe_type.connects_direction(&d) {
                //println!("\t connects to {:?}", d);
                let neighbour_pipe: Option<&Pipe> = self.get_neighbour_pipe_direction(&target_pipe, &d);
                if !neighbour_pipe.is_none() {
                    //println!("\t\tfound neighbour {:?}", neighbour_pipe);
                    if neighbour_pipe.unwrap().pipe_type.connects_direction(&d.opposite()) {
                        //println!("\t\t\tneighbour connects!");
                        neighbour_pipes.push(neighbour_pipe.unwrap());
                    }
                }
            }
        }

        return neighbour_pipes;
    }
}



fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let map: PipeMap = PipeMap::from_input_lines(&input_lines);
    let start_position: &Pipe = map.get_start_pipe().expect("No start position found");

    let mut neighbours: Vec<&Pipe> = map.get_neighbour_pipes(&start_position);
    let mut all_steps: Vec<usize> = Vec::new();
    
    for neighbour in neighbours {
        //println!("{:?}", neighbour);
        let mut finished = false;
        let mut new_neighbours;
        let mut steps = 1;
        let mut previous: &Pipe = start_position;
        let mut current_neighbour: &Pipe = neighbour;


        println!("STARTING NOW");
        while !finished {
            //println!("getting neighbours of {:?}", current_neighbour);
            //println!("(previous is {:?}", previous);
            new_neighbours = map.get_neighbour_pipes(current_neighbour);
            //println!("\t{:?}", new_neighbours);
            if new_neighbours.len() == 1 || (new_neighbours.contains(&start_position) && steps != 1) {
                finished = true;
            }

            // increment
            for n in new_neighbours {
                if n != previous {
                    previous = current_neighbour;
                    current_neighbour = n;
                    break;
                    //println!("\t(Next Step. previous = {:?}, current = {:?})", previous, current_neighbour);
                }
            }

            steps += 1;
        }
        all_steps.push(steps);
    }


    return all_steps.iter().max().expect("Steps vector is empty") / 2;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipes() {
        // Do a quick test here
        let input_lines: Vec<String> = generic::read_in_file((INPUTS_FOLDER.to_owned() + "/input_example_1.txt").as_str());
        let map: PipeMap = PipeMap::from_input_lines(&input_lines);

        assert!(map.get_pipe(1, 1).pipe_type.connects(&map.get_pipe(1, 2).pipe_type, &Direction::Right));
        assert!(map.get_pipe(1, 2).pipe_type.connects(&map.get_pipe(1, 3).pipe_type, &Direction::Right));
        assert!(map.get_pipe(1, 3).pipe_type.connects(&map.get_pipe(2, 3).pipe_type, &Direction::Down));
        assert!(map.get_pipe(2, 3).pipe_type.connects(&map.get_pipe(3, 3).pipe_type, &Direction::Down));
        assert!(map.get_pipe(3, 3).pipe_type.connects(&map.get_pipe(3, 2).pipe_type, &Direction::Left));
        assert!(map.get_pipe(3, 2).pipe_type.connects(&map.get_pipe(3, 1).pipe_type, &Direction::Left));
        assert!(map.get_pipe(3, 1).pipe_type.connects(&map.get_pipe(2, 1).pipe_type, &Direction::Up));
        assert!(map.get_pipe(2, 1).pipe_type.connects(&map.get_pipe(1, 1).pipe_type, &Direction::Up));


        assert!(map.get_pipe(1, 1).pipe_type.connects(&map.get_pipe(2, 1).pipe_type, &Direction::Down));
        assert!(map.get_pipe(2, 1).pipe_type.connects(&map.get_pipe(3, 1).pipe_type, &Direction::Down));
        assert!(map.get_pipe(3, 1).pipe_type.connects(&map.get_pipe(3, 2).pipe_type, &Direction::Right));
        assert!(map.get_pipe(3, 2).pipe_type.connects(&map.get_pipe(3, 3).pipe_type, &Direction::Right));
        assert!(map.get_pipe(3, 3).pipe_type.connects(&map.get_pipe(2, 3).pipe_type, &Direction::Up));
        assert!(map.get_pipe(2, 3).pipe_type.connects(&map.get_pipe(1, 3).pipe_type, &Direction::Up));
        assert!(map.get_pipe(1, 3).pipe_type.connects(&map.get_pipe(1, 2).pipe_type, &Direction::Left));
        assert!(map.get_pipe(1, 2).pipe_type.connects(&map.get_pipe(1, 1).pipe_type, &Direction::Left));
    }

    #[test]
    fn test_pipes2() {
        // Do a quick test here
        let input_lines: Vec<String> = generic::read_in_file((INPUTS_FOLDER.to_owned() + "/input_example_2.txt").as_str());
        let map: PipeMap = PipeMap::from_input_lines(&input_lines);
        let direction: Direction = Direction::Up;

        assert!(map.get_pipe(3, 1).pipe_type.connects(&map.get_pipe(2, 1).pipe_type, &direction) == false);
        
        let target_pipe = map.get_pipe(3,1);
        let neighbour_pipe: Option<&Pipe> = map.get_neighbour_pipe_direction(&target_pipe, &direction);

        assert!(neighbour_pipe.unwrap() == map.get_pipe(2,1));

        println!("{:?}", target_pipe);

        if target_pipe.pipe_type.connects_direction(&direction) {
            println!("Target pipe connects Up");
            let neighbour_pipe: Option<&Pipe> = map.get_neighbour_pipe_direction(&target_pipe, &direction);
            if !neighbour_pipe.is_none() {
                println!("\t\tfound neighbour {:?}", neighbour_pipe);
                if neighbour_pipe.unwrap().pipe_type.connects_direction(&direction.opposite()) {
                    println!("\t\t\tneighbour connects!");
                }
            }
        }
        
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 4);
    }

    #[test]
    fn example_2_part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 8);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 6903);
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