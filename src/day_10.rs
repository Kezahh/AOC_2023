const INPUTS_FOLDER: &str = "src/inputs/day_10";

#[warn(dead_code)]
use crate::generic;


#[derive(Debug, Eq, PartialEq, Clone)]
enum TileType {
    Pipe,
    Inside,
    Outside,
    None,
}

impl TileType {
    fn as_char(&self) -> char {
        match self {
            Self::Pipe => 'P',
            Self::Inside => 'I',
            Self::Outside => 'O',
            Self::None => '.',
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct TilePipeCount {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

impl TilePipeCount {
    fn blank() -> Self {
        return TilePipeCount {
            up: 0,
            down: 0,
            left: 0,
            right: 0,
        }
    }
}

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

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
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

    fn get_start_pipe_type(&self) -> PipeType {
        let start_pipe = self.get_start_pipe().unwrap();

        let all_directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];
        let mut neighbour_directions: Vec<Direction> = Vec::new();
        for d in all_directions {
            if start_pipe.pipe_type.connects_direction(&d) {
                let temp_neighbour = self.get_neighbour_pipe_direction(start_pipe, &d);
                if !temp_neighbour.is_none() {
                    if temp_neighbour.unwrap().pipe_type.connects_direction(&d.opposite()) {
                        neighbour_directions.push(d);
                    }
                }
            }
        }
        let neighbour_directions_tuple = (neighbour_directions[0].clone(), neighbour_directions[1].clone());
        //println!("\tNeighbour directions = {:?}", neighbour_directions);
        
        return match neighbour_directions_tuple {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => PipeType::Vertical,
            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => PipeType::NorthToWest,
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => PipeType::NorthToEast,
            (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => PipeType::SouthToWest,
            (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => PipeType::SouthToEast,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => PipeType::Horizontal,
            _ => PipeType::Ground,
        };
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

    let neighbours: Vec<&Pipe> = map.get_neighbour_pipes(&start_position);
    let mut all_steps: Vec<usize> = Vec::new();

    let mut all_pipe_parts: Vec<&Pipe> = Vec::new();
    
    //println!("{:?}", neighbour);
    let mut finished = false;
    let mut new_neighbours;
    let mut steps = 1;
    let mut previous: &Pipe = start_position;
    let mut current_neighbour: &Pipe = neighbours[0];

    all_pipe_parts.push(start_position);

    println!("STARTING NOW");
    while !finished {
        all_pipe_parts.push(current_neighbour);
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

    if part_2 {
        let mut tile_map: Vec<Vec<TileType>> = vec![vec![TileType::None; map.actual_map[0].len()]; map.actual_map.len()];
        let mut pipe_count_map: Vec<Vec<TilePipeCount>> = vec![vec![TilePipeCount::blank(); map.actual_map[0].len()]; map.actual_map.len()];

        for pipe in all_pipe_parts {
            tile_map[pipe.position.row][pipe.position.col] = TileType::Pipe;
        }

        // Get above and below
        for col in 0..tile_map[0].len() {
            let mut current_above_count: usize = 0;
            let mut bend_to_east: bool = false;
            let mut bend_to_west: bool = false;

            for row in 0..tile_map.len() {
                pipe_count_map[row][col].up = current_above_count;
                
                if tile_map[row][col] == TileType::Pipe {
                    let mut pipe_type: PipeType = map.get_pipe(row, col).pipe_type.clone();
                    if pipe_type == PipeType::StartPosition {
                        pipe_type = map.get_start_pipe_type();
                    }

                    match pipe_type {
                        PipeType::Vertical => current_above_count += 0,
                        PipeType::Horizontal => current_above_count += 1,
                        PipeType::NorthToEast | PipeType::SouthToEast => {
                            if bend_to_west {
                                current_above_count += 1;
                                bend_to_west = false;
                                bend_to_east = false;
                            } else if bend_to_east {
                                current_above_count += 0;
                                bend_to_east = false;
                            } else {
                                bend_to_east = true;
                            }
                        },
                        PipeType::NorthToWest | PipeType::SouthToWest => {
                            if bend_to_east {
                                current_above_count += 1;
                                bend_to_west = false;
                                bend_to_east = false;
                            } else if bend_to_west {
                                current_above_count += 0;
                                bend_to_west = false;
                            } else {
                                bend_to_west = true;
                            }
                        },
                        PipeType::StartPosition => (),
                        PipeType::Ground => (),
                    }
                }
            }

            for row in (0..tile_map.len()).rev() {
                pipe_count_map[row][col].down = current_above_count - pipe_count_map[row][col].up;
            }
        }

        // Get left and right
        for row in 0..tile_map.len() {
            let mut current_left_count: usize = 0;
            let mut bend_to_north: bool = false;
            let mut bend_to_south: bool = false;

            for col in 0..tile_map[0].len() {
                pipe_count_map[row][col].left = current_left_count;

                if tile_map[row][col] == TileType::Pipe {
                    let mut pipe_type: PipeType = map.get_pipe(row, col).pipe_type.clone();
                    if pipe_type == PipeType::StartPosition {
                        pipe_type = map.get_start_pipe_type();
                    }

                    match pipe_type {
                        PipeType::Vertical => current_left_count += 1,
                        PipeType::Horizontal => current_left_count += 0,
                        PipeType::NorthToEast | PipeType::NorthToWest => {
                            if bend_to_south {
                                current_left_count += 1;
                                bend_to_south = false;
                                bend_to_north = false;
                            } else if bend_to_north {
                                current_left_count += 0;
                                bend_to_north = false;
                            } else {
                                bend_to_north = true;
                            }
                        },
                        PipeType::SouthToEast | PipeType::SouthToWest => {
                            if bend_to_north {
                                current_left_count += 1;
                                bend_to_south = false;
                                bend_to_north = false;
                            } else if bend_to_south {
                                current_left_count += 0;
                                bend_to_south = false;
                            } else {
                                bend_to_south = true;
                            }
                        },
                        PipeType::StartPosition => (),
                        PipeType::Ground => (),
                    }
                }
            }

            let _all_left_side: Vec<usize> = pipe_count_map[row].iter().map(|x| x.left).collect::<Vec<usize>>();
            //println!("Beginning cols for row {}", row);
            //println!("\tcurrent_left_count = {}", current_left_count);
            //println!("\t{:?}", _all_left_side);

            for col in (0..tile_map[0].len()).rev() {
                //println!("\t\tSetting right for {} = {}", col, (current_left_count - pipe_count_map[row][col].left));
                pipe_count_map[row][col].right = current_left_count - pipe_count_map[row][col].left;
            }
        }


        for row in 0..tile_map.len() {
            for col in 0..tile_map[0].len() {
                if tile_map[row][col] != TileType::Pipe {
                    let pc = &pipe_count_map[row][col];
                    if pc.up % 2 == 1 && pc.down % 2 == 1 && pc.left % 2 == 1 && pc.right %2 == 1 {
                        tile_map[row][col] = TileType::Inside;
                    } else {
                        tile_map[row][col] = TileType::Outside;
                    }
                }
            }
        }

        let mut count_inside = 0;
        for row in 0..tile_map.len() {
            let mut row_string = String::new();
            // let mut other_row_string = String::new();
            for col in 0..tile_map[0].len() {
                row_string.push(tile_map[row][col].as_char());
                if tile_map[row][col] == TileType::Inside {
                    count_inside += 1;
                }
                // if tile_map[row][col] == TileType::Pipe {
                //     //row_string.push('P');
                //     row_string += format!("{}", pipe_count_map[row][col].left).as_str();
                //     other_row_string += format!("{}", pipe_count_map[row][col].right).as_str();
                // } else {
                //     row_string += format!("{}", pipe_count_map[row][col].left).as_str();
                //     other_row_string += format!("{}", pipe_count_map[row][col].right).as_str();
                // }
            }
            println!("{:?}", row_string);
            //println!("{:?}\t\t{:?}", row_string, other_row_string);
        }

        //println!("{:?}", map.get_pipe(6, 14));
        //println!("\t{:?}", pipe_count_map[6][14]);

        return count_inside;
    } else {
        return all_steps.iter().max().expect("Steps vector is empty") / 2;
    }
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
    fn example_1_part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1);
    }

    #[test]
    fn example_2_part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_2.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 1);
    }

    #[test]
    fn example_3_part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_3.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 4);
    }

    #[test]
    fn example_4_part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_4.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 8);
    }

    #[test]
    fn example_5_part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_5.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 10);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 265);
    }
}