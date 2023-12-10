use aoc_runner_derive::aoc;

fn match_spelled_out_digit(idx: usize, len: usize, line: &str) -> Option<i32> {
    if idx <= (len - 3) && len > 3 {
        let sl = &line[idx..idx + 3];
        match sl {
            "one" => return Some(1),
            "two" => return Some(2),
            "six" => return Some(6),
            _ => (),
        }
    }
    if idx <= (len - 4) && len > 4 {
        let sl = &line[idx..idx + 4];
        match sl {
            "four" => return Some(4),
            "five" => return Some(5),
            "nine" => return Some(9),
            _ => (),
        }
    }
    if idx <= (len - 5) && len > 5 {
        let sl = &line[idx..idx + 5];
        match sl {
            "eight" => return Some(8),
            "seven" => return Some(7),
            "three" => return Some(3),
            _ => (),
        }
    }
    None
}

fn get_last_digit_part2(line: &str) -> i32 {
    let len = line.len();
    let mut idx = len - 1;
    for c in line.chars().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as i32;
        }
        if let Some(value) = match_spelled_out_digit(idx, len, line) {
            return value;
        }
        idx -= 1;
    }
    panic!("No value for line")
}

fn get_first_digit_part2(line: &str) -> i32 {
    let len = line.len();
    for (idx, c) in line.chars().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as i32;
        }
        if let Some(value) = match_spelled_out_digit(idx, len, line) {
            return value;
        }
    }
    panic!("No value for line")
}

fn to_calibration_value_part2(line: &str) -> i32 {
    let first = get_first_digit_part2(line);
    let last = get_last_digit_part2(line);
    let value = format!("{}{}", first, last);
    value.parse::<i32>().unwrap()
}

#[aoc(day1, part2)]
fn part2(content: &str) -> i32 {
    content
        .lines()
        .map(|line: &str| to_calibration_value_part2(line))
        .sum()
    // 53312
}

fn to_calibration_value(line: &str) -> i32 {
    let first = line.chars().filter(|c| c.is_numeric()).next().unwrap();
    let last = line
        .chars()
        .rev()
        .filter(|c| c.is_numeric())
        .next()
        .unwrap();
    let value = format!("{}{}", first, last);
    value.parse::<i32>().unwrap()
}

#[aoc(day1, part1)]
fn part1(content: &str) -> i32 {
    content
        .lines()
        .map(|line: &str| to_calibration_value(line))
        .sum()
    // 53386
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const INPUT2: &str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 142);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT2), 281);
    }
}
