use std::collections::HashSet;

use aoc_runner_derive::aoc;

fn get_matches(card_line: &str) -> usize {
    card_line
        .split_once(':')
        .unwrap()
        .1
        .split(" | ")
        .map(|c: &str| -> HashSet<i32> {
            HashSet::from_iter(c.split(" ").filter_map(|c| c.parse::<i32>().ok()))
        })
        .reduce(|acc, e| &acc & &e)
        .unwrap()
        .len() as usize
}

#[aoc(day4, part2)]
fn part2(content: &str) -> u32 {
    let mut card_instances: Vec<u32> = vec![1u32; content.lines().count() + 1];

    for (card_id, card_line) in content.lines().enumerate() {
        let matches = get_matches(&card_line);
        if matches == 0 {
            continue;
        }

        let card_number = card_id + 1;
        for next_card_number in card_number + 1..=card_number + matches {
            card_instances[next_card_number] += card_instances[card_number];
        }
    }

    card_instances.iter().sum::<u32>() - 1u32
    // 5329815
}

#[aoc(day4, part1)]
fn part1(content: &str) -> u32 {
    content
        .lines()
        .map(get_matches)
        .filter(|w| w > &0)
        .map(|w| 2u32.pow(w as u32 - 1))
        .sum()
    // 21105
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 30);
    }
}
