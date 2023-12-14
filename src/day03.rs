use std::collections::HashSet;

use aoc_runner_derive::aoc;

struct Point {
    x: i32,
    y: i32,
}

struct Number {
    value: i32,
    length: i32,
    root: Point,
}

struct Symbol {
    char: char,
    point: Point,
}

fn parse_and_store_number(
    line: &str,
    number_root_x: usize,
    length: usize,
    numbers: &mut Vec<Number>,
    y: usize,
) {
    let value = (&line[number_root_x..number_root_x + length])
        .parse::<i32>()
        .unwrap();
    numbers.push(Number {
        value: value,
        length: length as i32,
        root: Point {
            x: number_root_x as i32,
            y: y as i32,
        },
    });
}

fn parse_numbers_and_symbols(content: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut parsing_number: bool;
    let mut number_root_x: usize;
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (y, line) in content.lines().enumerate() {
        parsing_number = false;
        number_root_x = 0;
        for (x, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if !parsing_number {
                    parsing_number = true;
                    number_root_x = x;
                }
            } else if parsing_number {
                let length = x - number_root_x;
                parse_and_store_number(line, number_root_x, length, &mut numbers, y);
                parsing_number = false;
            }
            if !ch.is_numeric() && ch != '.' {
                symbols.push(Symbol {
                    char: ch,
                    point: Point {
                        x: x as i32,
                        y: y as i32,
                    },
                });
            }
        }
        if parsing_number {
            let length = line.len() - number_root_x;
            parse_and_store_number(line, number_root_x, length, &mut numbers, y);
        }
    }
    (numbers, symbols)
}

fn number_near_symbol(number: &Number, symbol: &Symbol) -> bool {
    /* There should be a way to calculate this instead of doing a lame loop... eh */
    let (sx, sy) = (symbol.point.x, symbol.point.y);
    for i in 0..number.length {
        let digit = Point {
            x: number.root.x + i,
            y: number.root.y,
        };
        if {
            ((sx + 1) >= digit.x && digit.x >= (sx - 1))
                && ((sy + 1) >= digit.y && digit.y >= (sy - 1))
        } {
            return true;
        }
    }
    false
}

#[aoc(day3, part1)]
fn part1(content: &str) -> i32 {
    let (numbers, symbols): (Vec<Number>, Vec<Symbol>) = parse_numbers_and_symbols(content);

    let mut sum = 0;
    for number in numbers.iter() {
        for symbol in symbols.iter() {
            if number_near_symbol(number, symbol) {
                sum += number.value;
                break;
            }
        }
    }
    sum
    // 528799
}

#[aoc(day3, part2)]
fn part2(content: &str) -> i32 {
    let (numbers, symbols): (Vec<Number>, Vec<Symbol>) = parse_numbers_and_symbols(content);

    let mut sum = 0;
    let mut points_used: HashSet<(i32, i32)> = HashSet::new();
    let mut ratio_numbers: Vec<&Number> = Vec::new();
    for symbol in symbols.iter() {
        if symbol.char != '*' {
            continue;
        }

        ratio_numbers.clear();
        for number in numbers.iter() {
            if points_used.contains(&(number.root.x, number.root.y)) {
                continue;
            }
            if number_near_symbol(number, symbol) {
                ratio_numbers.push(number);
                if ratio_numbers.len() == 3 {
                    break;
                }
            }
        }

        if ratio_numbers.len() != 2 {
            continue;
        }
        sum += ratio_numbers
            .iter()
            .map(|number| {
                points_used.insert((number.root.x, number.root.y));
                number.value
            })
            .product::<i32>();
    }
    sum
    // 84907174
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 467835);
    }
}
