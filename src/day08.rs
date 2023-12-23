use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day8, part1)]
fn part1(content: &str) -> u32 {
    let instructions = content.lines().next().unwrap();
    let rules: Vec<&str> = content.lines().skip(2).collect();
    let mut mapping: HashMap<String, (String, String)> = HashMap::new();
    for rule in rules.iter() {
        let split: String = rule.replace(" ", "");
        let (from, left_right) = split.split_once('=').unwrap();
        let (left, right) = left_right.split_once(',').unwrap();
        mapping.insert(
            from.to_string(),
            (left[1..].to_string(), right[..right.len() - 1].to_string()),
        );
    }

    let mut position = String::from("AAA");
    let mut step = 0;
    loop {
        for instruction in instructions.chars() {
            let (left, right) = mapping.get(&position).unwrap();
            position = if instruction == 'L' {
                left.to_string()
            } else {
                right.to_string()
            };
            step += 1;
            if position == "ZZZ" {
                return step;
            }
        }
    }
    //19667
}

fn prime_factors(number: &u128) -> Vec<u128> {
    let mut i = 2;
    let mut factors: Vec<u128> = Vec::new();

    let mut n = *number;
    while i * i <= n {
        if (n % i) > 0 {
            i += 1;
        } else {
            n = n / i;
            factors.push(i);
        }
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}

#[aoc(day8, part2)]
fn part2(content: &str) -> u128 {
    let instructions = content.lines().next().unwrap();
    let rules: Vec<&str> = content.lines().skip(2).collect();
    let mut mapping: HashMap<String, (String, String)> = HashMap::new();

    let mut heads: Vec<String> = Vec::new();
    for rule in rules.iter() {
        let split: String = rule.replace(" ", "");
        let (from, left_right) = split.split_once('=').unwrap();
        let (left, right) = left_right.split_once(',').unwrap();

        if from.ends_with("A") {
            heads.push(from.to_string());
        }
        mapping.insert(
            from.to_string(),
            (left[1..].to_string(), right[..right.len() - 1].to_string()),
        );
    }

    let mut step = 0u128;
    let mut first_z_per_head: HashMap<String, u128> = HashMap::new();
    loop {
        for instruction in instructions.chars() {
            step += 1;
            for idx in 0..heads.len() {
                let (left, right) = mapping.get(&heads[idx]).unwrap();
                let position = if instruction == 'L' {
                    left.to_string()
                } else {
                    right.to_string()
                };
                if position.ends_with("Z") && !first_z_per_head.contains_key(&position) {
                    // Keep track of the first Z position per head
                    // The number of steps between a Z position for each head is constant
                    first_z_per_head.insert(position.to_string(), step);

                    // When we know the first_z for each head, we can calculate the step at which all heads will
                    // align on a Z position. This is the product of the prime factors of each number
                    if first_z_per_head.len() == heads.len() {
                        let mut unique_factors: HashSet<u128> = HashSet::new();
                        for first_z in first_z_per_head.values() {
                            for prime_factor in prime_factors(first_z) {
                                unique_factors.insert(prime_factor);
                            }
                        }
                        return unique_factors.iter().product();
                    }
                }
                heads[idx] = position;
            }
        }
    }
    // 19185263738117
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_1_input1() {
        assert_eq!(part1(&INPUT1), 6);
    }

    #[test]
    fn test_part_1_input2() {
        assert_eq!(part1(&INPUT2), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT3), 6);
    }
}
