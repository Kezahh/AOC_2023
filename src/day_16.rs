const INPUTS_FOLDER: &str = "src/inputs/day_16";

use std::collections::HashMap;

use crate::generic;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TileType {
    VerticalPipe,
    HorizontalPipe,
    ForwardSlash,
    BackwardSlash,
    Empty,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            '/' => Self::ForwardSlash,
            '\\' => Self::BackwardSlash,
            _ => Self::Empty,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Tile {
    tile_type: TileType,
    row: usize,
    col: usize,
    activated: bool,
    directions: HashMap<Direction, bool>,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        let mut directions_hashmap: HashMap<Direction, bool> = HashMap::new();
        directions_hashmap.insert(Direction::Up, false);
        directions_hashmap.insert(Direction::Down, false);
        directions_hashmap.insert(Direction::Left, false);
        directions_hashmap.insert(Direction::Right, false);
        return Self{tile_type: TileType::from(value), row: 0, col: 0, activated: false, directions: directions_hashmap};
    }
}

impl Tile {
    fn new(value: char, row: usize, col: usize) -> Self {
        let mut directions_hashmap: HashMap<Direction, bool> = HashMap::new();
        directions_hashmap.insert(Direction::Up, false);
        directions_hashmap.insert(Direction::Down, false);
        directions_hashmap.insert(Direction::Left, false);
        directions_hashmap.insert(Direction::Right, false);
        return Self{tile_type: TileType::from(value), row: row, col: col, activated: false, directions: directions_hashmap};
    }

    fn activate(&mut self, direction: Direction) -> Vec<Direction> {
        let mut return_directions: Vec<Direction> = Vec::new();
        if (self.directions[&direction]) {
            // Direction already set.
        } else {
            self.directions.insert(direction.clone(), true);
            self.activated = true;

            match self.tile_type {
                TileType::VerticalPipe => match direction {
                    Direction::Up => return_directions.push(direction.clone()),
                    Direction::Down => return_directions.push(direction.clone()),
                    Direction::Left | Direction::Right => {
                        return_directions.push(Direction::Up);
                        return_directions.push(Direction::Down);
                    },
                },
                TileType::HorizontalPipe => match direction {
                    Direction::Up | Direction::Down => {
                        return_directions.push(Direction::Left);
                        return_directions.push(Direction::Right);
                    },
                    Direction::Left => return_directions.push(direction.clone()),
                    Direction::Right => return_directions.push(direction.clone()),
                },
                TileType::ForwardSlash => match direction {
                    Direction::Up => return_directions.push(Direction::Left),
                    Direction::Down => return_directions.push(Direction::Right),
                    Direction::Left => return_directions.push(Direction::Up),
                    Direction::Right => return_directions.push(Direction::Down),
                }
                TileType::BackwardSlash => match direction {
                    Direction::Up => return_directions.push(Direction::Right),
                    Direction::Down => return_directions.push(Direction::Left),
                    Direction::Left => return_directions.push(Direction::Down),
                    Direction::Right => return_directions.push(Direction::Up),
                }
                TileType::Empty => return_directions.push(direction.clone()),
            }
            
            self.directions.insert(direction, true);
        }

        return return_directions;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    fn new(input_lines: Vec<String>) -> Self {
        let mut tile_map: Vec<Vec<Tile>> = input_lines.iter().enumerate()
            .map(|(row, x)| x.chars().enumerate()
            .map(|(col, y)| Tile::new(y, row, col)).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>();

        return Self{tiles: tile_map};
    }

    fn get_tile(&self, row: usize, col: usize) -> Tile {
        return self.tiles[row][col].clone();
    }

    fn row_count(&self) -> usize {
        return self.tiles.len();
    }

    fn col_count(&self) -> usize {
        return self.tiles[0].len();
    }

    fn activate_tile(&mut self, target_tile: &Tile, direction: Direction) -> Vec<Direction> {
        return self.tiles[target_tile.row][target_tile.col].activate(direction);
    }

    fn count_activated(&self) -> usize {
        let mut sum: usize = 0;
        for row in &self.tiles {
            sum += row.iter().filter(|x| x.activated).collect::<Vec<&Tile>>().len();
        }
        return sum;
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_filename.as_str());
    let mut tile_map: TileMap = TileMap::new(input_lines);

    let max_row: usize = tile_map.row_count();
    let max_col: usize = tile_map.col_count();

    if (!part_2) {
        return get_lava_count(tile_map.clone(), 0, 0, Direction::Left)
    } else {
        let mut all_counts: Vec<usize> = Vec::new();
        for row in 0..max_row {
            all_counts.push(get_lava_count(tile_map.clone(), row, 0, Direction::Left));
            all_counts.push(get_lava_count(tile_map.clone(), row, max_col - 1, Direction::Right));
        }
        println!("{:?}", all_counts);
        
        for col in 0..max_col {
            all_counts.push(get_lava_count(tile_map.clone(), 0, col, Direction::Down));
            all_counts.push(get_lava_count(tile_map.clone(), max_row - 1, col, Direction::Up));
        }
        println!("{:?}", all_counts);

        return *all_counts.iter().max().unwrap();
    }

}

fn get_lava_count(mut tile_map: TileMap, start_row: usize, start_col: usize, start_direction: Direction) -> usize {
    let max_row: i32 = tile_map.row_count() as i32;
    let max_col: i32 = tile_map.col_count() as i32;

    let mut leading_tiles: Vec<Tile> = Vec::new();
    let mut leading_tiles_directions: Vec<Direction> = Vec::new();

    leading_tiles.push(tile_map.get_tile(start_row, start_col));
    leading_tiles_directions.push(start_direction);

    while (leading_tiles.len() > 0) {
        let mut new_leading_tiles: Vec<Tile> = Vec::new();
        let mut new_leading_tiles_directions: Vec<Direction> = Vec::new();
        
        for i in 0..leading_tiles.len() {
            let tile: &Tile = &leading_tiles[i];
            let new_directions: Vec<Direction> = tile_map.activate_tile(tile, leading_tiles_directions[i].clone());

            for d in new_directions {
                let mut new_tile_row: i32 = tile.row as i32;
                let mut new_tile_col: i32 = tile.col as i32;

                match d {
                    Direction::Left => new_tile_col += 1,
                    Direction::Up => new_tile_row -= 1,
                    Direction::Down => new_tile_row += 1,
                    Direction::Right => new_tile_col -= 1, 
                }

                if new_tile_row < 0 || new_tile_col < 0 || new_tile_row >= max_row || new_tile_col >= max_col {
                    continue;
                } else {
                    new_leading_tiles.push(tile_map.get_tile(new_tile_row as usize, new_tile_col as usize));
                    new_leading_tiles_directions.push(d);
                }
            }
        }
        leading_tiles = new_leading_tiles;
        leading_tiles_directions = new_leading_tiles_directions;
    }

    return tile_map.count_activated();
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
        assert!(answer == 46);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 7477);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 51);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(INPUTS_FOLDER.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 7853);
    }
}