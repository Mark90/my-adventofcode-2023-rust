use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
fn part1(content: &str) -> u32 {
    content
        .lines()
        .map(|l| {
            let (first, second) = l.split_once(':').unwrap().1.split_once(" | ").unwrap();
            let winning: HashSet<i32> =
                HashSet::from_iter(first.split(" ").filter_map(|c| c.parse::<i32>().ok()));
            let mine: HashSet<i32> =
                HashSet::from_iter(second.split(" ").filter_map(|c| c.parse::<i32>().ok()));
            let my_winning_cards = winning.intersection(&mine).count() as u32;
            if my_winning_cards == 0 {
                0
            } else {
                2u32.pow(my_winning_cards - 1)
            }
        })
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
}
