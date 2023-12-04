#[warn(dead_code)]
use crate::generic;
use std::collections::HashMap;

fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);

    let mut games: Vec<HashMap<String, usize>> = Vec::new();

    for mut line in input_lines.clone() {
        //println!("{:?}", line);
        let mut new_game: HashMap<String, usize> = HashMap::new();
        line = line[line.find(":").unwrap()+2..].to_string();

        for game_turn in line.split("; ") {
            for colour_string in game_turn.split(", ") {
                let count = colour_string[..colour_string.find(" ").unwrap()].parse::<usize>().unwrap();
                let colour_name = colour_string[colour_string.find(" ").unwrap() + 1..].to_string();
                if new_game.contains_key(&colour_name) {
                    if count > *new_game.get(&colour_name).unwrap() {
                        new_game.insert(colour_name, count);
                    }
                } else {
                    new_game.insert(colour_name, count);
                }
            }
        }
        //println!("{:?}", new_game);
        games.push(new_game);
    }

    let mut target_cubes: HashMap<String, usize> = HashMap::new();
    target_cubes.insert("red".to_string(), 12);
    target_cubes.insert("green".to_string(), 13);
    target_cubes.insert("blue".to_string(), 14);

    let mut possible_games: Vec<usize> = Vec::new();

    for (i, game) in games.iter().enumerate() {
        let mut possible = true;
        for colour in game.keys() {
            if target_cubes.contains_key(colour) {
                possible = possible & (game.get(colour).unwrap() <= target_cubes.get(colour).unwrap());
            } else {
                possible = false;
            }
        }

        if possible {
            possible_games.push(i+1);
        }

        println!("{:?}", input_lines[i]);
        println!("{:?}", game);
        println!("Possible = {:?}", possible);
    }

    println!("{:?}", possible_games);


    if part_2 {
        let mut game_powers: Vec<usize> = Vec::new();
        for game in games {
            println!("{:?}", game);

            let game_power: usize = game.get("red").unwrap() * game.get("blue").unwrap() * game.get("green").unwrap();
            game_powers.push(game_power);
        }
        return game_powers.iter().sum();
    } else {
        return possible_games.iter().sum::<usize>();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let answer = solve_puzzle("src/inputs/day_2/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 8);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle("src/inputs/day_2/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 1931);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle("src/inputs/day_2/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 2286);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle("src/inputs/day_2/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 83105);
    }
}