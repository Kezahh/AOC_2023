const inputs_folder: &str = "src/inputs/day_7";

#[warn(dead_code)]
use crate::generic;
use std::collections::HashSet;
use std::cmp::{Ordering, PartialOrd};


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug,Eq,PartialEq, Clone)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: usize,
    joker_count: usize,
}

impl Hand {
    fn from_string(input_string: String) -> Self {
        let chars_list: Vec<char> = input_string[0..input_string.find(" ").expect("No space in input line")].chars().collect();
        let chars_set: HashSet<char> = HashSet::from_iter(chars_list.clone());
        let bid: usize = input_string[(input_string.find(" ").expect("No space in input line") + 1)..].parse::<usize>().expect("Bid is not a number");

        let hand_type: HandType;

        if chars_set.len() == 1 {
            // Five of a Kind
            hand_type = HandType::FiveOfAKind;
        } else if chars_set.len() == 4 {
            // One Pair
            hand_type = HandType::OnePair;
        } else if chars_set.len() == 5 {
            // High Card
            hand_type = HandType::HighCard;
        } else {
            // Four of a Kind
            // Full House
            // Three of a Kind
            // Two Pair
            let mut char_occurences: Vec<usize> = Vec::new();
            for c in chars_set {
                char_occurences.push(chars_list.iter().filter(|x| **x == c).count());
            }
            char_occurences.sort();
            char_occurences.reverse();

            if char_occurences.len() == 2 {
                if char_occurences[0] == 4 {
                    hand_type = HandType::FourOfAKind;
                } else {
                    hand_type = HandType::FullHouse;
                }
            } else {
                if char_occurences[0] == 3 {
                    hand_type = HandType::ThreeOfAKind;
                } else {
                    hand_type = HandType::TwoPair;
                }
            }
        }

        return Self { cards: chars_list.clone(), hand_type: hand_type, bid: bid, joker_count: chars_list.iter().filter(|x| **x == 'J').count()};
    }

    fn change_joker(&self) -> Self {
        let mut new_hand: Hand = self.clone();
        for i in 0..new_hand.cards.len() {
            if new_hand.cards[i] == 'J' {
                new_hand.cards[i] = '1';
            }
        }

        if new_hand.joker_count == 1 {
            match new_hand.hand_type {
                HandType::FiveOfAKind => (),
                HandType::FourOfAKind => new_hand.hand_type = HandType::FiveOfAKind,
                HandType::FullHouse => (),
                HandType::ThreeOfAKind => new_hand.hand_type = HandType::FourOfAKind,
                HandType::TwoPair => new_hand.hand_type = HandType::FullHouse,
                HandType::OnePair => new_hand.hand_type = HandType::ThreeOfAKind,
                HandType::HighCard => new_hand.hand_type = HandType::OnePair,
                _ => (),
            }
        } else if new_hand.joker_count == 2 {
            match new_hand.hand_type {
                HandType::FiveOfAKind => (),
                HandType::FourOfAKind => (),
                HandType::FullHouse => new_hand.hand_type = HandType::FiveOfAKind,
                HandType::ThreeOfAKind => (),
                HandType::TwoPair => new_hand.hand_type = HandType::FourOfAKind,
                HandType::OnePair => new_hand.hand_type = HandType::ThreeOfAKind,
                HandType::HighCard => (),
                _ => (),
            }
        } else if new_hand.joker_count == 3 {
            match new_hand.hand_type {
                HandType::FiveOfAKind => (),
                HandType::FourOfAKind => (),
                HandType::FullHouse => new_hand.hand_type = HandType::FiveOfAKind,
                HandType::ThreeOfAKind => new_hand.hand_type = HandType::FourOfAKind,
                HandType::TwoPair => (),
                HandType::OnePair => (),
                HandType::HighCard => (),
                _ => (),
            }
        } else if new_hand.joker_count == 4 {
            match new_hand.hand_type {
                HandType::FiveOfAKind => (),
                HandType::FourOfAKind => new_hand.hand_type = HandType::FiveOfAKind,
                HandType::FullHouse => (),
                HandType::ThreeOfAKind => (),
                HandType::TwoPair => (),
                HandType::OnePair => (),
                HandType::HighCard => (),
                _ => (),
            }
        }

        return new_hand;
    }
}

fn get_card_value(input_char: char) -> usize {
    if input_char.is_numeric() {
        return input_char.to_digit(10).expect("Bad char in get_card_value") as usize;
    } else {
        match input_char {
            'T' => return 10,
            'J' => return 11,
            'Q' => return 12,
            'K' => return 13,
            'A' => return 14,
            _ => return 0,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return get_card_value(self.cards[i]).cmp(&get_card_value(other.cards[i]));
                }
            }
            return Ordering::Equal;
        }
    }

    
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        } else {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return get_card_value(self.cards[i]).partial_cmp(&get_card_value(other.cards[i]));
                }
            }
            return Some(Ordering::Equal);
        }
    }
}


fn solve_puzzle(input_filename: String, part_2: bool) -> usize {
    let input_lines = generic::read_in_file(input_filename.as_str());
    let mut hands: Vec<Hand> = input_lines.iter().map(|x| Hand::from_string(x.to_string())).collect();

    if part_2 {
        // Account for Joker
        for i in 0..hands.len() {
            hands[i] = hands[i].change_joker();
        }
    }

    hands.sort();

    for hand in hands.iter() {
        println!("{:?}", hand);
    }

    let mut total_winnings: usize = 0;
    for i in 0..hands.len() {
        //println!("{:?}", hands[i]);
        total_winnings += hands[i].bid * (i + 1);
    }
    


    return total_winnings;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hands() {
        assert_eq!(Hand::from_string("AAAAA 0".to_string()).hand_type, HandType::FiveOfAKind);
        assert_eq!(Hand::from_string("AA8AA 0".to_string()).hand_type, HandType::FourOfAKind);
        assert_eq!(Hand::from_string("23332 0".to_string()).hand_type, HandType::FullHouse);
        assert_eq!(Hand::from_string("TTT98 0".to_string()).hand_type, HandType::ThreeOfAKind);
        assert_eq!(Hand::from_string("23432 0".to_string()).hand_type, HandType::TwoPair);
        assert_eq!(Hand::from_string("A23A4 0".to_string()).hand_type, HandType::OnePair);
        assert_eq!(Hand::from_string("23456 0".to_string()).hand_type, HandType::HighCard);

        assert!(Hand::from_string("AAAAA 0".to_string()) > Hand::from_string("AA8AA 0".to_string()));

        assert!(get_card_value('1') == 1);
        assert!(get_card_value('9') == 9);
        assert!(get_card_value('A') == 14);
        assert!(get_card_value('K') == 13);
        assert!(get_card_value('Q') == 12);
        assert!(get_card_value('J') == 11);
        assert!(get_card_value('T') == 10);

        assert!(Hand::from_string("KK677 28".to_string()) > Hand::from_string("KTJJT 220".to_string()));
    }

    #[test]
    fn example_1() {
        let answer = solve_puzzle(inputs_folder.to_owned() + "/input_example_1.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 6440);
    }

    #[test]
    fn part_1() {
        let answer = solve_puzzle(inputs_folder.to_owned() + "/input.txt", false);
        println!("Answer = {:?}", answer);
        assert!(answer == 252295678);
    }

    #[test]
    fn example_2() {
        let answer = solve_puzzle(inputs_folder.to_owned() + "/input_example_1.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 5905);
    }

    #[test]
    fn part_2() {
        let answer = solve_puzzle(inputs_folder.to_owned() + "/input.txt", true);
        println!("Answer = {:?}", answer);
        assert!(answer == 250577259);
    }
}