#[warn(dead_code)]
use crate::generic;


fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);



    return 0;
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
        let answer = solve_puzzle("src/inputs/day_4/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 13);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle("src/inputs/day_4/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 21138);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle("src/inputs/day_4/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 30);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle("src/inputs/day_4/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 7185540);
    }
}