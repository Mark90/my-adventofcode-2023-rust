use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn north(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn east(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn south(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn west(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(Clone)]
enum Shape {
    Horizontal, // -
    Vertical,   // |
    NorthEast,  // L
    NorthWest,  // J
    SouthEast,  // F
    SouthWest,  // 7
}

impl Shape {
    fn connectors(&self, position: &Position) -> (Position, Position) {
        match self {
            Shape::Horizontal => (position.west(), position.east()),
            Shape::Vertical => (position.north(), position.south()),
            Shape::SouthEast => (position.south(), position.east()),
            Shape::SouthWest => (position.south(), position.west()),
            Shape::NorthEast => (position.north(), position.east()),
            Shape::NorthWest => (position.north(), position.west()),
        }
    }
}

enum Tile {
    Ground,
    Pipe { pos: Position, shape: Shape },
}

// impl Tile {
//     fn is_connected_to(&self, other: &Tile) -> bool {
//         match (self, other) {
//             (Tile::Pipe { pos: p1, shape: s1 }, Tile::Pipe { pos: p2, shape: s2 }) => {
//                 let c1 = s1.connectors(&p1);
//                 let c2 = s2.connectors(&p2);

//                 let self_to_other = c1.0 == *p2 || c1.1 == *p2;
//                 let other_to_self = c2.0 == *p1 || c2.1 == *p1;
//                 self_to_other && other_to_self
//             }
//             _ => false,
//         }
//     }
// }

#[aoc(day10, part1)]
fn part1(content: &str) -> i32 {
    // // We'll need the grid dimensions later .. or do we?
    // let width = content
    //     .lines()
    //     .map(|l| l.len())
    //     .take(1)
    //     .collect::<Vec<usize>>()[0];
    // let height = content.lines().count();

    // Create grid of positions mapped to tiles (pipes/ground)
    // Track the animal's location and positions we can ignore
    let mut grid: HashMap<Position, Tile> = HashMap::new();
    let mut _animal: Option<Position> = None;
    let mut ignore: HashSet<Position> = HashSet::new();
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            let tile_shape = match c {
                '|' => Some(Shape::Vertical),
                '-' => Some(Shape::Horizontal),
                'L' => Some(Shape::NorthEast),
                'J' => Some(Shape::NorthWest),
                'F' => Some(Shape::SouthEast),
                '7' => Some(Shape::SouthWest),
                'S' => {
                    _animal = Some(pos.clone()); // record the position, figure out the shape later
                    None
                }
                _ => {
                    ignore.insert(pos.clone());
                    None
                }
            };
            let tile = match tile_shape {
                Some(shape) => Tile::Pipe {
                    pos: pos.clone(),
                    shape,
                },
                None => Tile::Ground,
            };
            grid.insert(pos, tile);
        }
    }

    // Derive shape of pipe where the animal resides.
    // Would not have been necessary with a sane input format...
    // Assume exactly 2 pipes connect to the animal's current tile - in my input this is the case
    let animal = _animal.unwrap();
    let matches: Vec<bool> = vec![animal.north(), animal.east(), animal.south(), animal.west()]
        .iter()
        .map(|direction| match grid.get(direction) {
            None => false,
            Some(tile) => match tile {
                Tile::Pipe { pos: p, shape: s } => {
                    let (c1, c2) = s.connectors(&p);
                    c1 == animal || c2 == animal
                }
                _ => false,
            },
        })
        .collect();

    let animal_pipe_shape = match matches[..] {
        // North East South West
        [false, true, false, true] => Shape::Horizontal,
        [true, false, true, false] => Shape::Vertical,
        [false, true, true, false] => Shape::SouthEast,
        [false, false, true, true] => Shape::SouthWest,
        [true, true, false, false] => Shape::NorthEast,
        [true, false, false, true] => Shape::NorthWest,
        _ => panic!("invalid directions"),
    };

    // There we go
    let animal_tile = Tile::Pipe {
        pos: animal.clone(),
        shape: animal_pipe_shape.clone(),
    };

    grid.insert(animal.clone(), animal_tile);

    // From the animal's current tile, traverse all pipe connectors until we're back at the start
    let (ac1, _) = animal_pipe_shape.connectors(&animal);
    let mut length = 0;
    let mut cur_pos: Position = ac1.clone();
    let mut prev_pos: Position = animal.clone();
    while cur_pos != animal {
        length += 1;
        let tile = grid.get(&cur_pos).unwrap();
        let (conn1, conn2) = match tile {
            Tile::Pipe { pos: tp, shape: ts } => ts.connectors(tp),
            Tile::Ground => panic!("there's not supposed to be ground here"),
        };
        let next_pos: Position = if conn1 != prev_pos { conn1 } else { conn2 };
        prev_pos = cur_pos;
        cur_pos = next_pos;
    }
    (length + 1) / 2

    // 6768
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 8);
    }
}
