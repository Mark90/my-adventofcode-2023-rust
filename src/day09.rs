use aoc_runner_derive::aoc;

fn next_value(values_: Vec<i64>) -> i64 {
    // Given a vector of integers following a certain trend, calculate the next integer
    let mut last_values: Vec<i64> = vec![];
    let mut values: Vec<i64> = values_.clone();
    loop {
        if values.iter().all(|v| v == &0) {
            return last_values.iter().sum();
        }
        last_values.push(values[values.len() - 1]);
        values = (1..values.len())
            .map(|i| values[i] - values[i - 1])
            .collect();
    }
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

fn next_backwards_history_value(line: &str) -> i64 {
    next_value(
        line.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .rev()
            .collect(),
    )
}

#[aoc(day9, part2)]
fn part2(content: &str) -> i64 {
    content.lines().map(next_backwards_history_value).sum()
    // 1140
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

    #[test]
    fn test_part_2() {
        assert_eq!(-5, part2("6 53 182 458 954 1719 2709 3660 3877 1908 -4933"));
        assert_eq!(19, part2("31 40 46 49 49 46"));
        assert_eq!(-159, part2("-178 -198 -219 -241"),);
        assert_eq!(2, part2(&INPUT));
        assert_eq!(
            1,
            part2("1 1 5 13 25 51 128 353 963 2541 6528 16414 40328 96357 222999 499077 1080929")
        );
        assert_eq!(
            12,
            part2("12 16 26 51 103 202 405 870 1965 4434 9640 19935 39296 74578 138165 253576")
        );
    }
}
