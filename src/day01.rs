use aoc_runner_derive::aoc;

fn to_calibration_value(line: &str) -> i32 {
    let first = line.chars().filter(|c| c.is_numeric()).next().unwrap();
    let last = line.chars().rev().filter(|c| c.is_numeric()).next().unwrap();
    let value = format!("{}{}",first,last);
    value.parse::<i32>().unwrap()
}

#[aoc(day1, part1)]
fn part1(content: &str) -> i32 {
    content.lines()
    .map(|line: &str| {
        to_calibration_value(line)
    }).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 142);
    }

}
