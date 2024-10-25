const INPUTS_FOLDER: &str = "src/inputs/day_17";

use crate::generic;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Point {
    row: usize,
    col: usize,
    value: usize,
    next_point: Vec<usize>,
    next_point_set: bool,
    path: Vec<Direction>,
}

impl Point {
    fn new(row: usize, col: usize, value: char) -> Self {
        return Self{
            row: row,
            col: col,
            value: value.to_digit(10).expect("Input is not a digit") as usize,
            next_point: vec![0,0],
            next_point_set: false,
            path: Vec::new(),
        }
    }
}

fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut points_map: Vec<Vec<Point>> = input_lines.iter().enumerate().map(|(row, x)| x.chars().enumerate().map(|(col, y)| Point::new(row, col, y)).collect::<Vec<Point>>()).collect::<Vec<Vec<Point>>>();

    for i in 0..1 {
        calc_distances_djikstra(&mut points_map);
    }

    println!("{:?}", points_map[points_map.len() - 1][points_map[0].len() - 1].path);
    return get_distance_to_start(points_map.len() - 1, points_map[0].len() - 1, &points_map);
}


fn calc_distances_djikstra(points_map: &mut Vec<Vec<Point>>) {
    println!("Starting Djikstra");
    points_map[0][0].next_point_set = true;
    for row_index in 0..points_map.len() {
        println!("Doing row {}", row_index);
        for col_index in 0..points_map[0].len() {
            println!("Doing col {}", col_index);
            if (row_index == (points_map.len() - 1) && col_index == (points_map[0].len() - 1)) {
                // Dont do for the last one.
                continue;
            }

            let mut neighbours: Vec<Vec<i32>> = Vec::new();
            let mut neighbours_directions: Vec<Direction> = Vec::new();

            if row_index > 0 {
                neighbours.push(vec![(row_index as i32) - 1, col_index as i32]);
                neighbours_directions.push(Direction::Up);
            }
            if row_index < points_map.len() - 1 {
                neighbours.push(vec![(row_index as i32) + 1, col_index as i32]);
                neighbours_directions.push(Direction::Down);
            }
            if col_index > 0 {
                neighbours.push(vec![row_index as i32, (col_index as i32) - 1]);
                neighbours_directions.push(Direction::Left);
            }
            if col_index < points_map[0].len() - 1 {
                neighbours.push(vec![row_index as i32, (col_index as i32) + 1]);
                neighbours_directions.push(Direction::Right);
            }

            let current_point = points_map[row_index][col_index].clone();
            println!("Current point = {:?}", current_point);
            println!("Neighbours = {:?}", neighbours);

            if !current_point.next_point_set {
                // current_point cannot connect to origin. No point telling neighbours.
                continue;
            }

            for (i, neighbour) in neighbours.iter().enumerate() {
                let current_point_distance = get_distance_to_start(current_point.row, current_point.col, &points_map);
                let neighbour_distance = get_distance_to_start(neighbour[0] as usize, neighbour[1] as usize, &points_map);
                let mut neighbour_point = &mut points_map[neighbour[0] as usize][neighbour[1] as usize];
                let neighbour_direction = neighbours_directions[i].clone();

                let path_len: usize = current_point.path.len();
                if path_len >= 3 {
                    if neighbour_direction == current_point.path[path_len - 1] 
                        && neighbour_direction == current_point.path[path_len - 2]
                        && neighbour_direction == current_point.path[path_len - 3] {
                        // If the current direction matches the last two directions, then that would be 3 in a row.
                        continue;
                    }
                }

                if !neighbour_point.next_point_set {
                    neighbour_point.next_point = vec![current_point.row, current_point.col];
                    neighbour_point.next_point_set = true;
                    neighbour_point.path = current_point.path.clone();
                    neighbour_point.path.push(neighbour_direction)

                } else {
                    if current_point_distance + neighbour_point.value < neighbour_distance {
                        neighbour_point.next_point = vec![current_point.row, current_point.col];
                        neighbour_point.path = current_point.path.clone();
                        neighbour_point.path.push(neighbour_direction)
                    }
                }
            }
        }
    }
}

fn get_distance_to_start(point_row: usize, point_col: usize, points_map: &Vec<Vec<Point>>) -> usize {
    let mut distance = 0;
    let mut current_point: Point = points_map[point_row][point_col].clone();
    while !(current_point.col == 0 && current_point.row == 0) {
        distance += current_point.value;
        current_point = points_map[current_point.next_point[0]][current_point.next_point[1]].clone();
    }

    return distance;
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
        println!("Here we go");
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 78);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 21138);
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