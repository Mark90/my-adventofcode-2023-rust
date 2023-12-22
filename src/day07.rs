use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOAK,
    FourOAK,
    FullHouse,
    ThreeOAK,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand {
    hand_type: HandType,
    strength: u32,
    bid: u32,
}

fn parse_line(raw_line: &str) -> Hand {
    let line = raw_line.split_whitespace().collect::<Vec<&str>>();

    // Group chars and count occurrences, sort counts and map that to handtype
    // I thought this would be really elegant, the end result is a bit so-so
    let mut groups: HashMap<char, u32> = HashMap::new();
    for c in line[0].chars() {
        groups.insert(c, groups.get(&c).unwrap_or(&0u32) + 1);
    }
    let mut counts: Vec<&u32> = groups.values().collect::<Vec<&u32>>();
    counts.sort_by(|a, b| b.cmp(&a));
    let hand_type = match counts[..] {
        [5] => HandType::FiveOAK,
        [4, 1] => HandType::FourOAK,
        [3, 2] => HandType::FullHouse,
        [3, 1, 1] => HandType::ThreeOAK,
        [2, 2, 1] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => panic!("invalid card format or the parsing is broken"),
    };

    // Convert str to hex (not using f and 0) then to int to get the hand strength
    //  AKQJT987654321 -> edcba987654321
    let hexadecimal: String = line[0]
        .chars()
        .map(|c| match c {
            'A' => 'e',
            'K' => 'd',
            'Q' => 'c',
            'J' => 'b',
            'T' => 'a',
            _ => c,
        })
        .collect();

    Hand {
        hand_type,
        strength: u32::from_str_radix(&hexadecimal, 16).unwrap(),
        bid: line[1].parse::<u32>().unwrap(),
    }
}

#[aoc(day7, part1)]
fn part1(content: &str) -> u32 {
    let mut hands: Vec<Hand> = content.lines().map(parse_line).collect();

    // Sort hands by Type (worst to best) then Strength (weak to strong)
    hands.sort_by(|a, b| {
        b.hand_type
            .cmp(&a.hand_type)
            .then(a.strength.cmp(&b.strength))
    });

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) as u32 * hand.bid)
        .sum()
    // 253313241
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 6440);
    }
}
