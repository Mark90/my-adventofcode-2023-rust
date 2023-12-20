use aoc_runner_derive::aoc;

fn to_numbers(line: &str) -> Vec<u32> {
    line.split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn part1(content: &str) -> u32 {
    let parsed: Vec<Vec<u32>> = content.lines().map(to_numbers).collect();
    let times = &parsed[0];
    let records = &parsed[1];

    (0..times.len())
        .map(|race_id| {
            (0..times[race_id])
                .map(|pressed| 0u32.max(times[race_id] - pressed) * pressed)
                .filter(|traveled| traveled > &records[race_id])
                .count() as u32
        })
        .product()
    // 1084752
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 288);
    }
}
