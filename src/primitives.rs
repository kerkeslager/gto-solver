#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Suit { internal: u8 }

impl Suit {
    pub fn from_char(c: char) -> Suit {
        Suit {
            internal: match c {
                'c' => 1,
                'd' => 2,
                'h' => 3,
                's' => 4,
                _ => panic!("Invalid suit character!")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rank { internal: u8 }

impl Rank {
    pub fn from_char(c: char) -> Rank {
        Rank {
            internal: match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => panic!("Invalid rank character!")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Card { internal: u8 }

impl Card {
    pub fn create(rank: Rank, suit: Suit) -> Card {
        Card { internal: (rank.internal << 4) | suit.internal }
    }

    pub fn from_chars(rank: char, suit: char) -> Card {
        Card::create( Rank::from_char(rank), Suit::from_char(suit))
    }

    pub fn from_str(s: &str) -> Card {
        assert!(s.len() == 2);
        let mut iter = s.chars();
        let r = iter.next().unwrap();
        let s = iter.next().unwrap();
        Card::from_chars(r, s)
    }

    pub fn suit(&self) -> Suit {
        Suit { internal: self.internal & 0x0f }
    }

    pub fn rank(&self) -> Rank {
        Rank { internal: (self.internal & 0xf0) >> 4 }
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn set_and_access_rank_and_suit() {
        let card = Card::from_str("Ks");
        assert_eq!(card.suit(), Suit::from_char('s'));
        assert_eq!(card.rank(), Rank::from_char('K'));
    }
}

pub struct HandStrength { score: u32 }

pub struct Hand { cards: [Card; 5] }

impl Hand {
    pub fn from_cards(a: Card, b: Card, c: Card, d: Card, e: Card) -> Hand {
        let mut cards = [ a, b, c, d, e ];

        // Sort cards by rank in reverse order, so aces are first
        cards.sort_by(|a, b| b.internal.cmp(&a.internal));

        Hand { cards: cards }
    }

    fn score(&self) -> u32 {
        let ranks = self.cards.map(|c| c.rank().internal);

        fn rank_score(&ranks: &[u8; 5]) -> u32 {
            let mut result = 0u32;
            for i in 0..5 {
                result <<= 4;
                result |= ranks[i] as u32;
            }
            result
        }

        let suits = self.cards.map(|c| c.suit().internal);
        let mut is_suited = true;
        for i in 1..5 {
            if suits[0] != suits[i] {
                is_suited = false;
                break;
            }
        }

        let mut is_straight = true;
        for i in 2..4 {
            if ranks[i] - 1 != ranks[i + 1] {
                is_straight = false;
                break;
            }
        }
        let is_wheel_straight = is_straight && ranks[0] == 14 && ranks[1] == 5;
        is_straight = is_straight && (is_wheel_straight || ranks[0] - 1 == ranks[1]);

        if is_suited {
            if is_straight {
                if is_wheel_straight {
                    return 0x9000_0005;
                }

                return 0x9000_0000 | ranks[0] as u32;
            }

            return 0x6000_0000 | rank_score(&ranks);
        }

        if is_straight {
            if is_wheel_straight {
                return 0x5000_0005;
            }

            return 0x5000_0000 | ranks[0] as u32;
        }

        let mut groups = [(0u8,0u8); 5];
        let mut group_index = 0;
        let mut group_size = 1;
        let mut group_rank = ranks[0];

        for i in 0..4 {
            if ranks[i] == ranks[i + 1] {
                group_size += 1
            } else {
                groups[group_index] = (group_size, group_rank);
                group_index += 1;
                group_size = 1;
                group_rank = ranks[i + 1];
            }
        }

        groups[group_index] = (group_size, group_rank);
        groups.sort_by(|a,b| (b.0).cmp(&a.0));

        fn group_score(groups: &[(u8,u8); 5]) -> u32 {
            let mut result = 0u32;

            for i in 0..5 {
                if groups[i].0 == 0 { break };
                result <<= 4;
                result |= groups[i].1 as u32;
            }

            result
        }

        match groups[0].0 {
            4 => 0x8000_0000 | group_score(&groups), // quads
            3 => {
                match groups[1].0 {
                    2 => 0x7000_0000 | group_score(&groups), // full house
                    _ => 0x4000_0000 | group_score(&groups), // trips
                }
            },
            2 => {
                match groups[1].0 {
                    2 => 0x3000_0000 | group_score(&groups), // two pair
                    _ => 0x2000_0000 | group_score(&groups), // pair
                }
            },
            _ => 0x1000_0000 | rank_score(&ranks),
        }
    }

    pub fn strength(&self) -> HandStrength {
        HandStrength { score: self.score() }
    }
}

#[cfg(test)]
mod hand_tests {
    use super::*;

    #[test]
    fn high_card() {
        let score = Hand::from_cards(
            Card::from_str("Ah"),
            Card::from_str("Qd"),
            Card::from_str("Jc"),
            Card::from_str("9s"),
            Card::from_str("3d"),
        ).score();

        assert_eq!(score, 0x100e_cb93);
    }

    #[test]
    fn pair() {
        let score = Hand::from_cards(
            Card::from_str("Ah"),
            Card::from_str("Qd"),
            Card::from_str("Jc"),
            Card::from_str("Js"),
            Card::from_str("3d"),
        ).score();

        assert_eq!(score, 0x2000_bec3);
    }

    #[test]
    fn two_pair() {
        let score = Hand::from_cards(
            Card::from_str("Ah"),
            Card::from_str("Qd"),
            Card::from_str("Qc"),
            Card::from_str("Js"),
            Card::from_str("Jd"),
        ).score();

        assert_eq!(score, 0x3000_0cbe);
    }

    #[test]
    fn trips() {
        let score = Hand::from_cards(
            Card::from_str("Ah"),
            Card::from_str("Qd"),
            Card::from_str("Qc"),
            Card::from_str("Qs"),
            Card::from_str("Jd"),
        ).score();

        assert_eq!(score, 0x4000_0ceb);
    }

    #[test]
    fn wheel_straight() {
        let score = Hand::from_cards(
            Card::from_str("Ah"),
            Card::from_str("2s"),
            Card::from_str("3d"),
            Card::from_str("4c"),
            Card::from_str("5d"),
        ).score();

        assert_eq!(score, 0x5000_0005);
    }

    #[test]
    fn straight() {
        let score = Hand::from_cards(
            Card::from_str("Ah"),
            Card::from_str("Ks"),
            Card::from_str("Qd"),
            Card::from_str("Jc"),
            Card::from_str("Td"),
        ).score();

        assert_eq!(score, 0x5000_000e);
    }

    #[test]
    fn flush() {
        let score = Hand::from_cards(
            Card::from_str("Ah"),
            Card::from_str("Qh"),
            Card::from_str("Jh"),
            Card::from_str("9h"),
            Card::from_str("3h"),
        ).score();

        assert_eq!(score, 0x600e_cb93);
    }

    #[test]
    fn full_house() {
        let score = Hand::from_cards(
            Card::from_str("Ah"),
            Card::from_str("Qd"),
            Card::from_str("Qc"),
            Card::from_str("Qs"),
            Card::from_str("Ad"),
        ).score();

        assert_eq!(score, 0x7000_00ce);
    }

    #[test]
    fn quads() {
        let score = Hand::from_cards(
            Card::from_str("Qh"),
            Card::from_str("Qd"),
            Card::from_str("Qc"),
            Card::from_str("Qs"),
            Card::from_str("Ad"),
        ).score();

        assert_eq!(score, 0x8000_00ce);
    }

    #[test]
    fn wheel_straight_flush() {
        let score = Hand::from_cards(
            Card::from_str("Ac"),
            Card::from_str("2c"),
            Card::from_str("3c"),
            Card::from_str("4c"),
            Card::from_str("5c"),
        ).score();

        assert_eq!(score, 0x9000_0005);
    }

    #[test]
    fn straight_flush() {
        let score = Hand::from_cards(
            Card::from_str("Ac"),
            Card::from_str("Kc"),
            Card::from_str("Qc"),
            Card::from_str("Jc"),
            Card::from_str("Tc"),
        ).score();

        assert_eq!(score, 0x9000_000e);
    }
}

#[cfg(test)]
mod size_tests {
    use super::*;

    #[test]
    fn rank() {
        assert_eq!(std::mem::size_of::<Rank>(), std::mem::size_of::<u8>());
    }

    #[test]
    fn suit() {
        assert_eq!(std::mem::size_of::<Suit>(), std::mem::size_of::<u8>());
    }

    #[test]
    fn card() {
        assert_eq!(std::mem::size_of::<Card>(), std::mem::size_of::<u8>());
    }

    #[test]
    fn hand_strength() {
        assert_eq!(std::mem::size_of::<HandStrength>(), std::mem::size_of::<u32>());
    }
}
