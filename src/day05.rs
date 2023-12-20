use std::ops::Range;

use aoc_runner_derive::aoc;

struct Rule {
    range: Range<i64>,
    addition: i64,
}

fn make_maps(sections: Vec<&str>) -> Vec<Vec<Rule>> {
    let mut maps = Vec::new();
    for section in sections {
        let mut mapping = Vec::new(); // TODO functional
        for line in section.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let dest = parts[0].parse::<i64>().unwrap();
            let src = parts[1].parse::<i64>().unwrap();
            let length = parts[2].parse::<i64>().unwrap();
            mapping.push(Rule {
                range: src..src + length,
                addition: dest - src,
            });
        }
        maps.push(mapping);
    }
    maps
}

#[aoc(day5, part1)]
fn part1(content: &str) -> i64 {
    let sections: Vec<&str> = content.split("\n\n").collect();

    let seeds_strings: Vec<&str> = sections[0]
        .split(": ")
        .skip(1)
        .map(|p| p.split_whitespace().collect::<Vec<&str>>())
        .last()
        .unwrap();
    let seeds: Vec<i64> = seeds_strings
        .iter()
        .map(|p| p.parse::<i64>().unwrap())
        .collect();

    let maps: Vec<Vec<Rule>> = make_maps(sections[1..].to_vec());

    let mut locations: Vec<i64> = Vec::new();
    for seed in seeds {
        // TODO functional
        let mut location = seed;
        for mapping in &maps {
            for rule in mapping {
                if rule.range.contains(&location) {
                    location = location + rule.addition;
                    break;
                }
            }
        }
        locations.push(location);
    }
    *locations.iter().min().unwrap()
    // 196167384
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 35);
    }
}
