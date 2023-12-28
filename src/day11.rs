use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn mhdist(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[aoc(day11, part1)]
fn part1(content: &str) -> i32 {
    let mut used_x_axis: HashSet<i32> = HashSet::new();
    let mut used_y_axis: HashSet<i32> = HashSet::new();

    // 1. Get initial galaxy coordinates
    let rawcoords2: Vec<Vec<Position>> = content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| ch == &'#')
                .map(|(x, _)| {
                    let p = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    used_x_axis.insert(p.x);
                    used_y_axis.insert(p.y);
                    p
                })
                .collect::<Vec<Position>>()
        })
        .collect::<Vec<Vec<Position>>>();
    let rawcoords = rawcoords2.into_iter().flatten().collect::<Vec<Position>>();

    // 2. Count all x and y expansions
    let y_expansions: Vec<i32> = (0..(content.lines().count()))
        .map(|y| y as i32)
        .filter(|y| !used_y_axis.contains(y))
        .collect::<Vec<i32>>();
    let first_line = content.lines().next().unwrap();

    let x_expansions: Vec<i32> = (0..(first_line.len()))
        .map(|x| x as i32)
        .filter(|x| !used_x_axis.contains(x))
        .collect::<Vec<i32>>();

    // 3. Shift galaxy coordinates based on expansions
    let finalcoords = rawcoords
        .iter()
        .map(|p| {
            let xshift = x_expansions.iter().filter(|x| *x < &p.x).count() as i32;
            let yshift = y_expansions.iter().filter(|y| *y < &p.y).count() as i32;
            Position {
                x: (p.x + xshift),
                y: (p.y + yshift),
            }
        })
        .collect::<Vec<Position>>();

    // 4. For all galaxy combinations...
    let mut combinations: HashSet<(&Position, &Position)> = HashSet::new();
    for galaxy in &finalcoords {
        for other in &finalcoords {
            if galaxy == other {
                continue;
            }
            let mut c: Vec<&Position> = Vec::new();
            c.push(galaxy);
            c.push(other);
            c.sort();
            combinations.insert((c[0], c[1]));
        }
    }

    // ... calculate the manhattan distance for each combination and return the sum
    combinations.iter().map(|(g1, g2)| g1.mhdist(g2)).sum()
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
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 374);
    }
}
