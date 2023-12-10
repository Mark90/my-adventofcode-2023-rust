use aoc_runner_derive::aoc;

#[aoc(day2, part2)]
fn part2(content: &str) -> i32 {
    content
        .lines()
        .map(|line| line.split_once(": ").unwrap().1)
        .map(|line: &str| {
            let rgb_max = get_rgb_max(line);
            rgb_max.0 * rgb_max.1 * rgb_max.2
        })
        .sum()
    // 63711
}

fn to_rgb(reveal: &str) -> (i32, i32, i32) {
    let reveal_ = reveal.strip_prefix(" ").or_else(|| Some(reveal)).unwrap();
    let (num_cubes_, color) = reveal_.split_once(' ').unwrap();
    let num_cubes = num_cubes_.parse::<i32>().unwrap();
    match color {
        "red" => (num_cubes, 0, 0),
        "green" => (0, num_cubes, 0),
        "blue" => (0, 0, num_cubes),
        _ => panic!("uhoh"),
    }
}

fn get_rgb_max(reveals_: &str) -> (i32, i32, i32) {
    let reveals: Vec<&str> = reveals_.split(&[',', ';']).collect();
    let rgb_vectors: Vec<(i32, i32, i32)> = reveals.iter().map(|reveal| to_rgb(reveal)).collect();
    rgb_vectors
        .iter()
        .copied()
        .reduce(|a: (i32, i32, i32), b: (i32, i32, i32)| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)))
        .unwrap()
}
fn get_id_if_valid_game(line: &str) -> i32 {
    let (game_id_, reveals) = line.split_once(": ").unwrap();
    let rgb_max = get_rgb_max(reveals);
    if rgb_max.0 <= 12 && rgb_max.1 <= 13 && rgb_max.2 <= 14 {
        (&game_id_[5..]).parse::<i32>().unwrap()
    } else {
        0
    }
}

#[aoc(day2, part1)]
fn part1(content: &str) -> i32 {
    content
        .lines()
        .map(|line: &str| get_id_if_valid_game(line))
        .sum()
    // 2439
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 2286);
    }
}
