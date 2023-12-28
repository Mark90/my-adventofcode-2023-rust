use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Position {
    x: i128,
    y: i128,
}

impl Position {
    fn mhdist(&self, other: &Position) -> i128 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn get_sum_of_shortest_paths(content: &str, expand_factor: i128) -> i128 {
    // 1. Get initial galaxy coordinates. Keep track of which x/y contain a galaxy
    let mut x_with_galaxy: HashSet<i128> = HashSet::new();
    let mut y_with_galaxy: HashSet<i128> = HashSet::new();
    let raw_galaxies_vec: Vec<Vec<Position>> = content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| ch == &'#')
                .map(|(x, _)| {
                    let pos = Position {
                        x: x as i128,
                        y: y as i128,
                    };
                    x_with_galaxy.insert(pos.x);
                    y_with_galaxy.insert(pos.y);
                    pos
                })
                .collect::<Vec<Position>>()
        })
        .collect::<Vec<Vec<Position>>>();
    // Flatten (wonder if .flatten_map() can be used to eliminate this step, not sure)
    let raw_galaxies: Vec<Position> = raw_galaxies_vec
        .into_iter()
        .flatten()
        .collect::<Vec<Position>>();

    // 2. Determine all x and y to expand
    let y_expansions: Vec<i128> = (0..(content.lines().count()))
        .map(|y| y as i128)
        .filter(|y| !y_with_galaxy.contains(y))
        .collect::<Vec<i128>>();
    let first_line = content.lines().next().unwrap();
    let x_expansions: Vec<i128> = (0..(first_line.len()))
        .map(|x| x as i128)
        .filter(|x| !x_with_galaxy.contains(x))
        .collect::<Vec<i128>>();

    // 3. Shift galaxy coordinates based on expansions
    let galaxies = raw_galaxies
        .iter()
        .map(|p| {
            let xshift = x_expansions.iter().filter(|x| *x < &p.x).count() as i128;
            let yshift = y_expansions.iter().filter(|y| *y < &p.y).count() as i128;
            Position {
                x: (p.x + (xshift * (expand_factor - 1).max(1))),
                y: (p.y + (yshift * (expand_factor - 1).max(1))),
            }
        })
        .collect::<Vec<Position>>();

    // 4. Generate all galaxy combinations...
    let mut combinations: HashSet<(&Position, &Position)> = HashSet::new();
    for galaxy in &galaxies {
        for other in &galaxies {
            if galaxy == other {
                continue;
            }
            // Create a tuple with the 2 galaxies sorted. This can probably be done in a much more performant way
            let mut combination: Vec<&Position> = Vec::new();
            combination.push(galaxy);
            combination.push(other);
            combination.sort();
            combinations.insert((combination[0], combination[1]));
        }
    }

    // ... and calculate manhattan distance for each combination and return the sum
    combinations.iter().map(|(g1, g2)| g1.mhdist(g2)).sum()
}

#[aoc(day11, part1)]
fn part1(content: &str) -> i128 {
    get_sum_of_shortest_paths(content, 1)
    // 9805264
}

#[aoc(day11, part2)]
fn part2(content: &str) -> i128 {
    get_sum_of_shortest_paths(content, 1_000_000)
    // 779032247216
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_get_sum_of_shortest_paths_part_1() {
        assert_eq!(get_sum_of_shortest_paths(&INPUT, 1), 374);
    }

    #[test]
    fn test_get_sum_of_shortest_paths_part_2() {
        assert_eq!(get_sum_of_shortest_paths(&INPUT, 10), 1030);
        assert_eq!(get_sum_of_shortest_paths(&INPUT, 100), 8410);
    }
}
