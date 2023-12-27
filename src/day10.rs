use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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
    // Track the animal's location
    let mut grid: HashMap<Position, Tile> = HashMap::new();
    let mut _animal: Option<Position> = None;
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
                _ => None,
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

#[aoc(day10, part2)]
fn part2(content: &str) -> i32 {
    // Create grid of positions mapped to tiles (pipes/ground)
    // Track the animal's location
    let mut grid: HashMap<Position, Tile> = HashMap::new();
    let mut _animal: Option<Position> = None;
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
                _ => None,
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
    let mut cur_pos: Position = ac1.clone();
    let mut prev_pos: Position = animal.clone();
    let mut main_pipe: HashSet<Position> = HashSet::new();
    while cur_pos != animal {
        main_pipe.insert(prev_pos.clone());
        main_pipe.insert(cur_pos.clone());

        let tile = grid.get(&cur_pos).unwrap();
        let (conn1, conn2) = match tile {
            Tile::Pipe { pos: tp, shape: ts } => ts.connectors(tp),
            Tile::Ground => panic!("there's not supposed to be ground here"),
        };
        let next_pos: Position = if conn1 != prev_pos { conn1 } else { conn2 };
        prev_pos = cur_pos;
        cur_pos = next_pos;
    }

    let width = content
        .lines()
        .map(|l| l.len())
        .take(1)
        .collect::<Vec<usize>>()[0] as i32;
    let height = content.lines().count() as i32;

    let mut count_enclosed = 0;

    for y in 0..height {
        let mut currently_enclosed = false;
        let mut last_curve_shape: Option<Shape> = None;
        for x in 0..width {
            let pos = Position { x, y };
            let tile = grid.get(&pos).unwrap();
            if currently_enclosed {
                match tile {
                    Tile::Ground => {
                        count_enclosed += 1;
                        println!("{:?} inc count to {} for ground", pos, count_enclosed);

                    }
                    Tile::Pipe {
                        pos: tpos,
                        shape: current_shape,
                    } => {
                        if !main_pipe.contains(&tpos) {
                            // junk pipe
                            count_enclosed += 1;
                            println!("{:?} inc count to {} for junk {:?}", pos, count_enclosed, current_shape);
                            continue;
                        }
                        if current_shape == &Shape::Horizontal {
                            // Continuing a curve
                            continue;
                        }
                        if current_shape == &Shape::Vertical {
                            // this ends the current enclosure
                            last_curve_shape = None;
                            currently_enclosed = false;
                            println!("{:?} end enclosure with shape {:?}", pos, current_shape);

                            continue;
                        }

                        // Current shape is a curve ..

                        let last_curve = match last_curve_shape {
                            Some(i) => i, // We've already seen a curve
                            None => {
                                // Not yet in a curve, store it and move on
                                last_curve_shape = Some(current_shape.clone());
                                continue;
                            }
                        };

                        let zigzag = match (&last_curve, current_shape) {
                            (Shape::NorthEast, Shape::SouthWest) => true,
                            (Shape::SouthEast, Shape::NorthWest) => true,
                            _ => false,
                        };

                        if zigzag {
                            // L7, L---7, FJ, F---J, etc are zigzags that function the same as a |
                            last_curve_shape = None;
                            currently_enclosed = false;
                            println!("{:?} end enclosure with zigzag shape {:?} {:?}", pos, last_curve, current_shape);

                            continue;
                        }

                        println!("{:?} Last shape {:?} and current shape {:?} form a U-turn -> do not end enclosure, reset last shape", pos, last_curve, current_shape);
                        last_curve_shape = None;
                    }
                }
            } else {
                // Not enclosed
                match tile {
                    Tile::Ground => continue,
                    Tile::Pipe {
                        pos: tpos,
                        shape: current_shape,
                    } => {
                        if !main_pipe.contains(&tpos) {
                            // junk pipe
                            continue;
                        }
                        if current_shape == &Shape::Horizontal {
                            // Continuing a curve
                            continue;
                        }
                        if current_shape == &Shape::Vertical {
                            // this starts a new enclosure
                            println!("{:?} start enclosure with shape {:?}", pos, current_shape);

                            currently_enclosed = true;
                            continue;
                        }

                        // Current shape is a curve ..
                        let last_curve = match last_curve_shape {
                            Some(i) => i, // We've already seen a curve
                            None => {
                                // Not yet in a curve, store it and move on
                                last_curve_shape = Some(current_shape.clone());
                                continue;
                            }
                        };

                        let zigzag = match (&last_curve, current_shape) {
                            (Shape::NorthEast, Shape::SouthWest) => true,
                            (Shape::SouthEast, Shape::NorthWest) => true,
                            _ => false,
                        };

                        if zigzag {
                            // L7, L---7, FJ, F---J, etc are zigzags that function the same as a |
                            last_curve_shape = None;
                            println!("{:?} start enclosure with zigzag shape {:?} {:?}", pos, last_curve, current_shape);
                            currently_enclosed = true;
                            continue;
                        }
                        println!("{:?} Last shape {:?} and current shape {:?} form a U-turn -> do not start enclosure, reset last_shape", pos, last_curve, current_shape);
                        last_curve_shape = None;
                    }
                }
            }
        }
    }
    count_enclosed
    // 351
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

    #[test]
    fn test_part_2_example1() {
        assert_eq!(
            part2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            4
        );
    }

    #[test]
    fn test_part_2_example1squeezed() {
        assert_eq!(
            part2(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            ),
            4
        );
    }

    #[test]
    fn test_part_2_example2() {
        assert_eq!(
            part2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );
    }

    #[test]
    fn test_part_2_example3() {
        assert_eq!(
            part2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );
    }
}
