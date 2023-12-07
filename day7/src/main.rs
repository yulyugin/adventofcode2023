use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("Task1 answer: {}", task1::handle_input(&input));
    // println!("Task2 answer: {}", task2::handle_input(&input));

    Ok(())
}

mod task1 {
    use core::cmp::{Ordering, PartialOrd};
    use std::collections::HashMap;
    use std::iter::zip;
    use std::mem::transmute;

    pub fn handle_input(input: &str) -> u32 {
        let mut hands = input.lines().map(|l| Hand::new(l)).collect::<Vec<Hand>>();
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            6440
        );
    }

    #[derive(PartialEq, Eq)]
    struct Hand {
        hand_type: HandType,
        cards: Vec<Card>,
        bid: u32,
    }

    impl Hand {
        fn new(input: &str) -> Self {
            let (cards, bid) = input.trim().split_once(" ").unwrap();
            let cards = cards.chars().map(|c| Card::from_char(c)).collect();
            let bid = bid.parse::<u32>().unwrap();
            let hand_type = HandType::from_cards(&cards);
            Self {
                hand_type,
                cards,
                bid,
            }
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.hand_type == other.hand_type {
                for (s, o) in zip(&self.cards, &other.cards) {
                    if s < o {
                        return Ordering::Less;
                    } else if s > o {
                        return Ordering::Greater;
                    }
                }
                return Ordering::Equal;
            }
            return self.hand_type.cmp(&other.hand_type);
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[test]
    fn test_hand_compare() {
        assert!(Hand::new("32T3K 765") < Hand::new("T55J5 684"));
        assert!(Hand::new("KK677 28") > Hand::new("KTJJT 220"));
        assert!(Hand::new("T55J5 684") < Hand::new("QQQJA 483"));
    }

    #[test]
    fn test_hand() {
        let hand = Hand::new("32T3K 765");
        assert_eq!(
            hand.cards,
            vec![Card::Three, Card::Two, Card::T, Card::Three, Card::K]
        );
        assert_eq!(hand.bid, 765);
        assert_eq!(hand.hand_type, HandType::OnePair);

        let hand = Hand::new("T55J5 684");
        assert_eq!(
            hand.cards,
            vec![Card::T, Card::Five, Card::Five, Card::J, Card::Five]
        );
        assert_eq!(hand.bid, 684);
        assert_eq!(hand.hand_type, HandType::ThreeOfKind);

        let hand = Hand::new("KK677 28");
        assert_eq!(
            hand.cards,
            vec![Card::K, Card::K, Card::Six, Card::Seven, Card::Seven]
        );
        assert_eq!(hand.bid, 28);
        assert_eq!(hand.hand_type, HandType::TwoPairs);

        let hand = Hand::new("KTJJT 220");
        assert_eq!(
            hand.cards,
            vec![Card::K, Card::T, Card::J, Card::J, Card::T]
        );
        assert_eq!(hand.bid, 220);
        assert_eq!(hand.hand_type, HandType::TwoPairs);

        let hand = Hand::new("QQQJA 483");
        assert_eq!(
            hand.cards,
            vec![Card::Q, Card::Q, Card::Q, Card::J, Card::A]
        );
        assert_eq!(hand.bid, 483);
        assert_eq!(hand.hand_type, HandType::ThreeOfKind);
    }

    #[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPairs,
        ThreeOfKind,
        FullHouse,
        FourOfKind,
        FiveOfKind,
    }

    impl HandType {
        fn from_cards(cards: &Vec<Card>) -> HandType {
            assert!(cards.len() == 5);
            let mut count: HashMap<Card, u32> = HashMap::new();
            for c in cards {
                match count.get_mut(c) {
                    Some(e) => {
                        *e += 1;
                    }
                    None => {
                        count.insert(c.clone(), 1);
                    }
                }
            }
            match count.len() {
                1 => HandType::FiveOfKind,
                2 => {
                    let values: Vec<u32> = count.into_values().collect();
                    if values[0] == 1 || values[0] == 4 {
                        HandType::FourOfKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    let mut values: Vec<u32> = count.into_values().collect();
                    values.sort();
                    if values[2] == 3 {
                        HandType::ThreeOfKind
                    } else {
                        HandType::TwoPairs
                    }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => panic!("unreachable"),
            }
        }
    }

    #[test]
    fn test_type_from_cards() {
        assert_eq!(
            HandType::from_cards(&vec![Card::A; 5]),
            HandType::FiveOfKind
        );
        assert_eq!(
            HandType::from_cards(&vec![Card::A, Card::A, Card::Eight, Card::A, Card::A]),
            HandType::FourOfKind
        );
        assert_eq!(
            HandType::from_cards(&vec![
                Card::Two,
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Two
            ]),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::from_cards(&vec![Card::T, Card::T, Card::T, Card::Nine, Card::Eight]),
            HandType::ThreeOfKind
        );
        assert_eq!(
            HandType::from_cards(&vec![
                Card::Two,
                Card::Three,
                Card::Four,
                Card::Three,
                Card::Two
            ]),
            HandType::TwoPairs
        );
        assert_eq!(
            HandType::from_cards(&vec![Card::A, Card::Two, Card::Three, Card::A, Card::Four]),
            HandType::OnePair
        );
        assert_eq!(
            HandType::from_cards(&vec![
                Card::Two,
                Card::Three,
                Card::Four,
                Card::Five,
                Card::Six
            ]),
            HandType::HighCard
        );
    }

    #[test]
    fn test_type_comparison() {
        assert!(HandType::HighCard < HandType::OnePair);
    }

    #[allow(dead_code)]
    #[repr(u32)]
    #[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
    enum Card {
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Six = 6,
        Seven = 7,
        Eight = 8,
        Nine = 9,
        T,
        J,
        Q,
        K,
        A,
    }

    impl Card {
        fn from_char(value: char) -> Self {
            match value {
                'T' => Self::T,
                'J' => Self::J,
                'Q' => Self::Q,
                'K' => Self::K,
                'A' => Self::A,
                value if value.is_digit(10) => unsafe { transmute(value.to_digit(10).unwrap()) },
                _ => panic!("unreachable"),
            }
        }
    }

    #[test]
    fn test_card_comparison() {
        assert!(Card::T > Card::Nine);
        assert!(Card::T < Card::J);
    }

    #[test]
    fn test_card_values() {
        assert_eq!(Card::T as u32, 10);
        assert_eq!(Card::J as u32, 11);
        assert_eq!(Card::Q as u32, 12);
        assert_eq!(Card::K as u32, 13);
        assert_eq!(Card::A as u32, 14);
    }

    #[test]
    fn test_from_char() {
        assert_eq!(Card::from_char('1'), Card::One);
        assert_eq!(Card::from_char('2'), Card::Two);
        assert_eq!(Card::from_char('3'), Card::Three);
        assert_eq!(Card::from_char('4'), Card::Four);
        assert_eq!(Card::from_char('5'), Card::Five);
        assert_eq!(Card::from_char('6'), Card::Six);
        assert_eq!(Card::from_char('7'), Card::Seven);
        assert_eq!(Card::from_char('8'), Card::Eight);
        assert_eq!(Card::from_char('9'), Card::Nine);
        assert_eq!(Card::from_char('T'), Card::T);
        assert_eq!(Card::from_char('J'), Card::J);
        assert_eq!(Card::from_char('Q'), Card::Q);
        assert_eq!(Card::from_char('K'), Card::K);
        assert_eq!(Card::from_char('A'), Card::A);
    }
}
