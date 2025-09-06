#[derive(Copy, Debug, PartialEq)]
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

#[derive(Copy, Debug, PartialEq)]
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

#[derive(Copy, Debug, PartialEq)]
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
mod tests {
    use super::*;

    #[test]
    fn set_and_access_rank_and_suit() {
        let card = Card::from_str("Ks");
        assert_eq!(card.suit(), Suit::from_char('s'));
        assert_eq!(card.rank(), Rank::from_char('K'));
    }
}
