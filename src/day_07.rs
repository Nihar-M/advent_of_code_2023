pub const DAY_STR: &str = "day_07";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::{
        cmp::{self, Ordering},
        collections::HashMap,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Card {
        Ace,
        King,
        Queen,
        Jack,
        Ten,
        Nine,
        Eight,
        Seven,
        Six,
        Five,
        Four,
        Three,
        Two,
        Joker,
    }

    impl From<char> for Card {
        fn from(value: char) -> Self {
            match value {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                '*' => Card::Joker,
                _ => panic!("Failed to parse char into Card"),
            }
        }
    }

    impl Card {
        fn card_value(&self) -> u32 {
            match &self {
                Card::Ace => 14,
                Card::King => 13,
                Card::Queen => 12,
                Card::Jack => 11,
                Card::Ten => 10,
                Card::Nine => 9,
                Card::Eight => 8,
                Card::Seven => 7,
                Card::Six => 6,
                Card::Five => 5,
                Card::Four => 4,
                Card::Three => 3,
                Card::Two => 2,
                Card::Joker => 1,
            }
        }
    }

    impl Ord for Card {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.card_value().cmp(&other.card_value())
        }
    }
    impl PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Hand(Card, Card, Card, Card, Card);

    impl From<&str> for Hand {
        fn from(value: &str) -> Self {
            // dbg!(value);
            let mut cards = value.chars().map(Card::from);

            Hand(
                cards.next().expect("Could not find card 1"),
                cards.next().expect("Could not find card 2"),
                cards.next().expect("Could not find card 3"),
                cards.next().expect("Could not find card 4"),
                cards.next().expect("Could not find card "),
            )
        }
    }

    #[test]
    fn from_str_hand() {
        assert_eq!(
            dbg!(Hand::from("AAAAT")),
            Hand(Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ten)
        )
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPairs,
        OnePair,
        HighCard,
    }

    impl HandType {
        fn strength(&self) -> u32 {
            match &self {
                HandType::FiveOfAKind => 6,
                HandType::FourOfAKind => 5,
                HandType::FullHouse => 4,
                HandType::ThreeOfAKind => 3,
                HandType::TwoPairs => 2,
                HandType::OnePair => 1,
                HandType::HighCard => 0,
            }
        }
    }

    impl Ord for HandType {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.strength().cmp(&other.strength())
        }
    }
    impl PartialOrd for HandType {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Hand {
        pub fn hand_type(&self) -> HandType {
            let mut map = HashMap::new();
            map.insert(self.0, 1);

            if let Some(count) = map.get_mut(&self.1) {
                *count += 1;
            } else {
                map.insert(self.1, 1);
            }
            if let Some(count) = map.get_mut(&self.2) {
                *count += 1;
            } else {
                map.insert(self.2, 1);
            }
            if let Some(count) = map.get_mut(&self.3) {
                *count += 1;
            } else {
                map.insert(self.3, 1);
            }
            if let Some(count) = map.get_mut(&self.4) {
                *count += 1;
            } else {
                map.insert(self.4, 1);
            }
            // dbg!(&map);

            let mut num_jokers_removed = 0;

            if let Some(num_jokers) = &map.get(&Card::Joker) {
                num_jokers_removed = **num_jokers;

                map.remove(&Card::Joker);

                // dbg!(num_jokers_removed);
            }

            let mut counts = map.into_values().collect::<Vec<_>>();

            // for _ in 0..num_jokers_removed {
            //     counts.push(1);
            // }

            // requested by clippy
            counts.resize(counts.len() + num_jokers_removed, 1);

            counts.sort();

            // dbg!(&counts);

            // if num_jokers_removed > 0 {
            //     todo!()
            // }

            let mut c_iter = counts.into_iter().rev();

            match (
                c_iter.next(),
                c_iter.next(),
                c_iter.next(),
                c_iter.next(),
                c_iter.next(),
            ) {
                (Some(5), None, None, None, None) => HandType::FiveOfAKind,
                (Some(4), Some(1), None, None, None) => HandType::FourOfAKind,
                (Some(3), Some(2), None, None, None) => HandType::FullHouse,
                (Some(3), Some(1), Some(1), None, None) => HandType::ThreeOfAKind,
                (Some(2), Some(2), Some(1), None, None) => HandType::TwoPairs,
                (Some(2), Some(1), Some(1), Some(1), None) => HandType::OnePair,
                (Some(1), Some(1), Some(1), Some(1), Some(1)) => HandType::HighCard,
                x => {
                    panic!("This case should be unreachable. Found {:?}", x)
                }
            }
            // todo!()
        }

        pub fn hand_type_with_joker(&self) -> HandType {
            let num_jokers = [self.0, self.1, self.2, self.3, self.4]
                .iter()
                .filter(|c| **c == Card::Joker)
                .count();

            let hand_type = self.hand_type();

            // match hand_type {
            //     HandType::FiveOfAKind => todo!(),
            //     HandType::FourOfAKind => todo!(),
            //     HandType::FullHouse => todo!(),
            //     HandType::ThreeOfAKind => todo!(),
            //     HandType::TwoPairs => todo!(),
            //     HandType::OnePair => todo!(),
            //     HandType::HighCard => todo!(),
            // }

            match num_jokers {
                0 => hand_type,
                1 => match hand_type {
                    HandType::HighCard => HandType::OnePair,
                    HandType::OnePair => HandType::ThreeOfAKind,
                    HandType::TwoPairs => HandType::FullHouse,
                    HandType::ThreeOfAKind => HandType::FourOfAKind,
                    HandType::FullHouse => HandType::FourOfAKind,
                    _ => HandType::FiveOfAKind,
                },
                2 => match hand_type {
                    HandType::HighCard => HandType::ThreeOfAKind,
                    HandType::OnePair => HandType::FourOfAKind,
                    HandType::TwoPairs => HandType::FourOfAKind,
                    _ => HandType::FiveOfAKind,
                },
                3 => match hand_type {
                    HandType::HighCard => HandType::FourOfAKind,
                    _ => HandType::FiveOfAKind,
                },
                4 => HandType::FiveOfAKind,
                5 => HandType::FiveOfAKind,
                x => {
                    panic!("Cannot have more than 5 jokers. Found {x:?}");
                }
            }
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            // match self.hand_type().cmp(&other.hand_type()) { // for part 1
            match self
                .hand_type_with_joker()
                .cmp(&other.hand_type_with_joker())
            {
                // for part 2
                Ordering::Equal => {}
                neq => return neq,
            }

            match self.0.cmp(&other.0) {
                Ordering::Equal => {}
                neq => return neq,
            }

            match self.1.cmp(&other.1) {
                Ordering::Equal => {}
                neq => return neq,
            }

            match self.2.cmp(&other.2) {
                Ordering::Equal => {}
                neq => return neq,
            }

            match self.3.cmp(&other.3) {
                Ordering::Equal => {}
                neq => return neq,
            }

            match self.4.cmp(&other.4) {
                Ordering::Equal => {}
                neq => return neq,
            }

            Ordering::Equal
        }
    }
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[test]
    fn hand_strength() {
        let mut hands = vec![
            Hand::from("32T3K"),
            Hand::from("T55J5"),
            Hand::from("KK677"),
            Hand::from("KTJJT"),
            Hand::from("QQQJA"),
            Hand::from("QQQJ2"),
        ];
        hands.sort();
        assert_eq!(
            hands,
            vec![
                Hand::from("32T3K"),
                Hand::from("KTJJT"),
                Hand::from("KK677"),
                Hand::from("T55J5"),
                Hand::from("QQQJ2"),
                Hand::from("QQQJA"),
            ]
        );
    }

    pub fn solution(input: String) -> u32 {
        let mut hand_n_bids = input
            .lines()
            .map(|line| line.split_at(6))
            .map(|(cards, bid)| {
                (
                    Hand::from(&cards[0..5]),
                    bid.parse::<u32>().expect("Failed to parse bid to u32"),
                )
            })
            .collect::<Vec<_>>();

        // dbg!(&hand_n_bids);
        hand_n_bids.sort_by(|a, b| a.0.cmp(&b.0));
        // dbg!(&hand_n_bids);

        let x = hand_n_bids
            .iter()
            .enumerate()
            .map(|(index, (_hand, bid))| {
                let rank = index as u32 + 1;
                rank * bid
            })
            .sum::<u32>();
        // dbg!(x);
        // todo!()
        x
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            6440
        );
    }
}

mod part_2 {
    use super::part_1::Hand;

    pub fn solution(input: String) -> u32 {
        let mut hand_n_bids = input
            .lines()
            .map(|line| line.split_at(6))
            .map(|(cards, bid)| {
                let cards = cards.replace('J', "*"); // replace jacks with jokers
                (
                    Hand::from(&cards[0..5]),
                    bid.parse::<u32>().expect("Failed to parse bid to u32"),
                )
            })
            .inspect(|(_hand, _bid)| {
                // dbg!(hand.hand_type());
            })
            .collect::<Vec<_>>();

        // dbg!(&hand_n_bids);
        hand_n_bids.sort_by(|a, b| a.0.cmp(&b.0));
        // dbg!(&hand_n_bids);

        let x = hand_n_bids
            .iter()
            .enumerate()
            .map(|(index, (_hand, bid))| {
                let rank = index as u32 + 1;
                rank * bid
            })
            .sum::<u32>();
        // dbg!(x);
        // todo!()
        x
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            5905
        );
    }

    #[test]
    fn hand_strength_with_jokers() {
        let mut hands: Vec<Hand> = vec![
            Hand::from("32T3K"),
            Hand::from("T55*5"),
            Hand::from("KK677"),
            Hand::from("KT**T"),
            Hand::from("QQQ*A"),
        ];
        hands.sort();
        dbg!(&hands);
        assert_eq!(
            hands,
            vec![
                Hand::from("32T3K"),
                Hand::from("KK677"),
                Hand::from("T55*5"),
                Hand::from("QQQ*A"),
                Hand::from("KT**T"),
            ]
        );
    }
}

// too high    251627483
// mistake in mapping TwoPairs + 2 Jokers to Full House

// too low     251342183
// used jokers to make OnePair,
// then used them again to upgrade that into a four of a kind

// just right  251515496
