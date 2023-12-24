#[warn(dead_code)]
use crate::generic;


#[derive(Clone, Debug, Eq, PartialEq)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn get_times(&self) -> Vec<usize> {
        // x is the time to press the button
        // speed = x
        // distance = (race.time - x) * speed
        //          = x*race.time - x^2
        //
        // x^2 - x*race.time - distance = 0
        //
        let time: f64 = self.time as f64;
        let distance: f64 = self.distance as f64;
        let mut first_time: f64 = (time + (time.powi(2) - (4.0*distance)).powf(0.5)) / 2.0;
        let mut second_time: f64 = (time - (time.powi(2) - (4.0*distance)).powf(0.5)) / 2.0;

        println!("first = {:?}", first_time);
        println!("second = {:?}", second_time);

        // check if exact
        if first_time.fract() == 0.0 {
            first_time -= 1.0;
        }
        if second_time.fract() == 0.0 {
            second_time += 1.0;
        }

        let first_time_rounded: usize = first_time.floor() as usize;
        let second_time_rounded: usize = second_time.ceil() as usize;

        return vec![second_time_rounded, first_time_rounded];
    }

    fn get_ways_to_win(&self) -> usize {
        let race_times: Vec<usize> = self.get_times();
        return (race_times[1] - race_times[0] + 1);
    }
}


fn races_from_input_lines(input_lines: Vec<String>) -> Vec<Race> {
    let mut output_races: Vec<Race> = Vec::new();
    let input_times: Vec<usize> = input_lines[0]
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|x| x.parse::<usize>().expect(format!("Input '{}' not a number.", x).as_str()))
        .collect();
    let input_distances: Vec<usize> = input_lines[1]
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()[1..]
        .iter()
        .map(|x| x.parse::<usize>().expect(format!("Input '{}' not a number.", x).as_str()))
        .collect();

    for i in 0..input_times.len() {
        output_races.push(Race { time: input_times[i], distance: input_distances[i] });
    }

    return output_races;
}

fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let races: Vec<Race>;

    if !part_2 {
        // PART ONE
        races = races_from_input_lines(input_lines);
    } else {
        // PART TWO
        let time = input_lines[0].chars().filter(|x| x.is_numeric()).collect::<String>().parse::<usize>().expect("Oh no bad time");
        let distance = input_lines[1].chars().filter(|x| x.is_numeric()).collect::<String>().parse::<usize>().expect("Oh no bad distance");

        races = vec![Race{time: time, distance: distance}];
    }

    let mut ways_to_win: Vec<usize> = Vec::new();
    let mut final_product: usize = 1;

    for race in races {
        let number_of_wins = race.get_ways_to_win();

        println!("{:?}", race);
        println!("{:?}", race.get_times());
        println!("{:?}", number_of_wins);
        ways_to_win.push(number_of_wins);
        final_product *= number_of_wins;
    }

    return final_product;
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
        let answer = solve_puzzle("src/inputs/day_6/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 288);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle("src/inputs/day_6/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 2065338);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle("src/inputs/day_6/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 71503);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle("src/inputs/day_6/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 34934171);
        
        // 34934174 is too high
    }
}