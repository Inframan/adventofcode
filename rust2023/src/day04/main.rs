use std::collections::{LinkedList, HashSet};

struct Card {
    id: u32,
    given_numbers: LinkedList<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    fn winning_matches(&self) -> u32 {
        return self.given_numbers.iter()
            .map(|g| match self.winning_numbers.contains(g) {
                true => 1,
                false => 0,
            })
            .sum()
    }

    fn total_points(&self) -> u32 {
        match self.winning_matches() {
            0 => 0,
            n => 2_u32.pow(n-1),
        }
    }    
}

fn main() {

    let cards = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards = parse_cards(cards);

    let total_points: u32 = cards
        .iter()
        .map(|c| {
            let points = c.total_points();
            println!("Card {} has {} points!", c.id, points);
            points
        })
        .sum();

    println!("The total number of points is: {total_points}");
}

// Checks if it is adjacent to any symbol
fn parse_cards(cards: &str) -> Vec<Card> {
    return cards.lines()
        .map(|l| parse_card(l))
        .collect::<Vec<Card>>();
}

fn parse_card(card: &str) -> Card {

    let mut split = card.split(":");

    let id: u32 = split.next()
        .expect(format!("Unable to get card id: {card}").as_str())[5..]
        .parse()
        .expect(format!("Unable parse card id to number: {card}").as_str());

    let numbers = split.next()
        .expect(format!("Unable to get card numbers: {card}").as_str());

    let mut numbers = numbers
        .split("|")
        .map(|nums| nums
            .split(" ")
            .map(|n| n.parse::<u32>())
            .filter(|r| r.is_ok())
            .map(|n| n.unwrap())
        );

    let given_numbers = numbers
        .next()
        .expect(format!("Unable to get given numbers: {card}").as_str())
        .collect::<LinkedList<u32>>();

    let winning_numbers = numbers
        .next()
        .expect(format!("Unable to get winning numbers: {card}").as_str())
        .collect::<HashSet<u32>>();

    return Card {
        id,
        given_numbers,
        winning_numbers,
    };
}

