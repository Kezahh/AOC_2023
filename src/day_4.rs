#[warn(dead_code)]
use crate::generic;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Card {
    index: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
    copies: usize,
}

impl Card {
    fn get_winning_matches(&self) -> usize {
        let mut winning_numbers_count: usize = 0;
        for num in &self.numbers {
            if self.winning_numbers.contains(&num) {
                winning_numbers_count += 1;
            }
        }

        return winning_numbers_count;
    }

    fn get_worth(&self) -> usize {
        let winning_numbers_count: usize = self.get_winning_matches();
        if winning_numbers_count > 0 {
            return usize::pow(2,(winning_numbers_count - 1) as u32);
        } else {
            return 0;
        }
    }
}

fn solve_puzzle(input_filename: &str, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename);
    let mut all_cards: Vec<Card> = get_all_cards(&input_lines);

    for i in 0..all_cards.len() {
        for j in 0..all_cards[i].get_winning_matches() {
            for k in 0..all_cards[i].copies {
                if (j + i) < (all_cards.len() - 1) {
                    all_cards[j + i + 1].copies += 1;
                }
            }
        }
        //println!("Card {} has {} winners", all_cards[i].index, all_cards[i].get_winning_matches());
        //println!("{:?}", all_cards.iter().map(|x| x.copies).collect::<Vec<usize>>());
    }

    for card in &all_cards {
        println!("card {:?} has {} copies", card.index, card.copies);
    }

    if !part_2 {
        return all_cards.iter().map(|x| x.get_worth()).sum();
    } else {
        return all_cards.iter().map(|x| x.copies).sum();
    }
}

fn get_all_cards(input_lines: &Vec<String>) -> Vec<Card> {
    let mut all_cards: Vec<Card> = Vec::new();

    for line in input_lines {
        let all_numbers = line[line.find(":").unwrap() + 1..].to_string();
        let winning_numbers_string: &str = all_numbers.split("|").collect::<Vec<&str>>()[0].trim();
        let numbers_string: &str = all_numbers.split("|").collect::<Vec<&str>>()[1].trim();
        
        all_cards.push(Card { 
            index: line["Card ".len()..line.find(":").unwrap()].trim().parse::<usize>().unwrap(),
            winning_numbers: HashSet::from_iter(winning_numbers_string.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<usize>().unwrap())), 
            numbers: HashSet::from_iter(numbers_string.split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<usize>().unwrap())),
            copies: 1,
        });
    }

    return all_cards;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_test() {
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