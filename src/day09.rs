use aoc_runner_derive::aoc;

fn next_value(values_: Vec<i64>) -> i64 {
    let mut last_values: Vec<i64> = vec![];
    let mut values = values_.clone();
    loop {
        let n = values.len();
        last_values.push(values[n - 1]);
        if n < 2 {
            break;
        }
        values = (1..values.len())
            .map(|i| values[i] - values[i - 1])
            .collect();
    }
    last_values.iter().sum()
}

fn next_history_value(line: &str) -> i64 {
    next_value(
        line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect(),
    )
}

#[aoc(day9, part1)]
fn part1(content: &str) -> i64 {
    content.lines().map(next_history_value).sum()
    // 1819125966
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        assert_eq!(part1("19 31 40 46 49 49"), 46);
        assert_eq!(part1("0 0"), 0);
        assert_eq!(part1("-1 -1"), -1);
        assert_eq!(part1("9 6 2 -3"), -9);
        assert_eq!(part1("-159 -178 -198 -219"), -241);
        assert_eq!(part1("12 21 35 61 126 285 637 1370 2868 5929 12171 24755 49646 97785 188781 357080 662060 1204173 2150147 3771417"), 6501422);
        assert_eq!(part1(&INPUT), 114);
    }
}
