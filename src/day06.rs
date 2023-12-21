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

#[aoc(day6, part2)]
fn part2(content: &str) -> u64 {
    let parsed: Vec<u64> = content
        .replace(" ", "")
        .lines()
        .map(|l| l.split_once(':').unwrap().1.parse::<u64>().unwrap())
        .collect();
    let race_duration = parsed[0];
    let record_distance = parsed[1];

    for press_duration in 0..race_duration {
        let traveled = (race_duration - press_duration) * press_duration;
        if traveled > record_distance {
            return race_duration - (2 * press_duration) + 1;
        }
    }
    panic!("not found");
    // 28228952
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

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 71503);
    }
}
