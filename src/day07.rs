use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn new(input_line: &str, wildcard_jacks: bool) -> Hand {
        let split = input_line.split_whitespace().collect::<Vec<&str>>();
        let cards = split[0]
            .chars()
            .map(|x| match x {
                'T' => 10,
                'J' => {
                    if wildcard_jacks {
                        1
                    } else {
                        11
                    }
                }
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => x.to_digit(10).unwrap(),
            })
            .collect::<Vec<u32>>();
        let cards_set = cards
            .iter()
            .filter(|x| !wildcard_jacks || **x != 1)
            .collect::<std::collections::HashSet<&u32>>();
        let hand_type = match cards_set.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                let mut hand_type = HandType::TwoPair;
                for card in cards_set {
                    if cards
                        .iter()
                        .filter(|&x| x == card || (wildcard_jacks && x == &1))
                        .count()
                        == 3
                    {
                        hand_type = HandType::ThreeOfAKind;
                        break;
                    }
                }
                hand_type
            }
            2 => {
                let mut hand_type = HandType::FullHouse;
                for card in cards_set {
                    if cards
                        .iter()
                        .filter(|&x| x == card || (wildcard_jacks && x == &1))
                        .count()
                        == 4
                    {
                        hand_type = HandType::FourOfAKind;
                        break;
                    }
                }
                hand_type
            }
            1 => HandType::FiveOfAKind,
            0 => {
                if wildcard_jacks {
                    HandType::FiveOfAKind
                } else {
                    panic!("Invalid number of cards")
                }
            }
            _ => panic!("Invalid number of cards"),
        };
        Hand {
            cards,
            hand_type,
            bid: split[1].parse::<u32>().unwrap(),
        }
    }
}

fn cmp_hands(a: &Hand, b: &Hand) -> Ordering {
    match a.hand_type.cmp(&b.hand_type) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            for card_index in 0..5 {
                match a.cards[card_index].cmp(&b.cards[card_index]) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                }
            }
            Ordering::Equal
        }
        Ordering::Less => Ordering::Less,
    }
}

pub(crate) fn day07() {
    let f: File = File::open("data/day07.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let mut part1_hands: Vec<Hand> = lines
        .iter()
        .map(|x| Hand::new(x, false))
        .collect::<Vec<Hand>>();
    part1_hands.sort_by(cmp_hands);
    println!(
        "Day 1 part 1: {}",
        part1_hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + x.bid * (i as u32 + 1))
    );

    let mut part2_hands: Vec<Hand> = lines
        .iter()
        .map(|x| Hand::new(x, true))
        .collect::<Vec<Hand>>();
    part2_hands.sort_by(cmp_hands);
    println!(
        "Day 1 part 2: {}",
        part2_hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + x.bid * (i as u32 + 1))
    );
}
