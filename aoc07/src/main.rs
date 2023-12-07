use std::{cmp::Ordering, collections::HashMap, error::Error, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("{}", find_total_winnings(&input)?);
    Ok(())
}

fn find_total_winnings(s: &str) -> Result<usize, Box<dyn Error>> {
    let mut parsed = s
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let hand = iter.next().ok_or("missing hand")?.parse::<Hand>()?;
            let bid = iter.next().ok_or("missing bid")?.parse::<usize>()?;
            Ok::<_, Box<dyn Error>>((hand, bid))
        })
        .collect::<Result<Vec<_>, _>>()?;
    parsed.sort_by_key(|(hand, _bid)| hand.clone());
    Ok(parsed
        .into_iter()
        .enumerate()
        .map(|(rank_from_zero, (_hand, bid))| (rank_from_zero + 1) * bid)
        .sum())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Card {
    Digit(u8),
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let card_map = self
            .cards
            .iter()
            .fold(HashMap::<Card, usize>::new(), |mut map, &el| {
                *map.entry(el).or_default() += 1;
                map
            });

        if card_map.len() == 1 {
            HandType::FiveOfAKind
        } else if card_map.iter().any(|(_k, &v)| v == 4) {
            HandType::FourOfAKind
        } else if card_map.iter().any(|(_k, &v)| v == 3) && card_map.iter().any(|(_k, &v)| v == 2) {
            HandType::FullHouse
        } else if card_map.iter().any(|(_k, &v)| v == 3) {
            HandType::ThreeOfAKind
        } else if card_map
            .iter()
            .find(|(_k, &v)| v == 2)
            .is_some_and(|(&k1, _v)| card_map.iter().any(|(&k2, &v2)| k1 != k2 && v2 == 2))
        {
            HandType::TwoPair
        } else if card_map.iter().any(|(_k, &v)| v == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(&s, &o)| s.cmp(&o))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| match c {
                'A' => Ok(Card::A),
                'K' => Ok(Card::K),
                'Q' => Ok(Card::Q),
                'J' => Ok(Card::J),
                'T' => Ok(Card::T),
                _ if c.is_ascii_digit() => Ok(Card::Digit(c.to_digit(10).unwrap() as u8)),
                _ => Err(format!("Invalid card: {c}")),
            })
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap();
        Ok(Hand { cards })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_orders_cards_correctly() {
        assert!(Card::A > Card::K);
        assert!(Card::T > Card::Digit(5));
        assert!(Card::Digit(9) > Card::Digit(5));
    }

    #[test]
    fn hands_have_correct_type() {
        assert_eq!(
            HandType::FullHouse,
            Hand {
                cards: [Card::T, Card::T, Card::K, Card::K, Card::K]
            }
            .hand_type()
        );

        const ONE: Card = Card::Digit(1);
        assert_eq!(HandType::FiveOfAKind, Hand { cards: [ONE; 5] }.hand_type());
        assert_eq!(
            HandType::FourOfAKind,
            Hand {
                cards: [ONE, ONE, ONE, ONE, Card::Digit(2)]
            }
            .hand_type()
        );
    }

    #[test]
    fn it_orders_hands_correctly() {
        assert!(
            Hand {
                cards: [Card::A, Card::A, Card::A, Card::A, Card::A]
            } > Hand {
                cards: [Card::T, Card::T, Card::A, Card::A, Card::A]
            }
        );
        assert!(
            Hand {
                cards: [Card::A, Card::A, Card::A, Card::A, Card::A]
            } > Hand {
                cards: [Card::Q, Card::Q, Card::Q, Card::Q, Card::Q]
            }
        );
    }

    #[test]
    fn it_works_1() {
        let s = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, find_total_winnings(s).unwrap());
    }
}
