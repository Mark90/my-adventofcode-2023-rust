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

#[derive(Clone, PartialEq)]
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
        /* Given a position, return the a tuple of 2 positions to the pipe's connectors  */
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

fn create_grid(content: &str) -> (HashMap<Position, Tile>, Position, Position) {
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
    let animal: Position = _animal.unwrap();
    let direction_matches: Vec<bool> =
        vec![animal.north(), animal.east(), animal.south(), animal.west()]
            .iter()
            .map(|direction| match grid.get(direction) {
                None => false,
                Some(tile) => match tile {
                    Tile::Pipe { pos: p, shape: s } => {
                        let (connector1, connector2) = s.connectors(&p);
                        connector1 == animal || connector2 == animal
                    }
                    _ => false,
                },
            })
            .collect();

    let animal_pipe_shape = match direction_matches[..] {
        // North East South West
        [false, true, false, true] => Shape::Horizontal,
        [true, false, true, false] => Shape::Vertical,
        [false, true, true, false] => Shape::SouthEast,
        [false, false, true, true] => Shape::SouthWest,
        [true, true, false, false] => Shape::NorthEast,
        [true, false, false, true] => Shape::NorthWest,
        _ => panic!("invalid direction matches for animal tile"),
    };

    // Add pipe on animal's tile to the grid and select one of its connectors as next position
    let animal_tile = Tile::Pipe {
        pos: animal.clone(),
        shape: animal_pipe_shape.clone(),
    };
    grid.insert(animal.clone(), animal_tile);
    let (next_position, _) = animal_pipe_shape.connectors(&animal);

    (grid, animal, next_position)
}

fn discover_main_loop(
    grid: &HashMap<Position, Tile>,
    animal: &Position,
    next_position: Position,
) -> HashSet<Position> {
    // From the animal's current tile, traverse all pipe connectors until we're back at the start
    let mut cur_pos: Position = next_position.clone();
    let mut prev_pos: Position = animal.clone();
    let mut main_loop: HashSet<Position> = HashSet::new();
    while cur_pos != *animal {
        main_loop.insert(prev_pos.clone());
        main_loop.insert(cur_pos.clone());
        let tile = (*grid).get(&cur_pos).unwrap();
        let (conn1, conn2) = match tile {
            Tile::Pipe {
                pos: next_pos,
                shape: next_shape,
            } => next_shape.connectors(next_pos),
            Tile::Ground => panic!("there's not supposed to be ground here"),
        };
        let next_pos: Position = if conn1 != prev_pos { conn1 } else { conn2 };
        prev_pos = cur_pos;
        cur_pos = next_pos;
    }
    main_loop.clone()
}

#[aoc(day10, part1)]
fn part1(content: &str) -> i32 {
    let (grid, animal, next_position): (HashMap<Position, Tile>, Position, Position) =
        create_grid(content);

    let main_loop = discover_main_loop(&grid, &animal, next_position);

    main_loop.len() as i32 / 2
    // 6768
}

#[aoc(day10, part2)]
fn part2(content: &str) -> i32 {
    let (grid, animal, next_position): (HashMap<Position, Tile>, Position, Position) =
        create_grid(content);

    let main_loop = discover_main_loop(&grid, &animal, next_position);

    let width = content
        .lines()
        .map(|l| l.len())
        .take(1)
        .collect::<Vec<usize>>()[0] as i32;
    let height = content.lines().count() as i32;

    let mut count_enclosed = 0;

    // Traverse the grid line by line. When crossing a vertical pipe that is part of the main loop, every
    // subsequent ground or junk-pipe is enclosed by the loop. When encountering another vertical tile,
    // subsequent tiles are not enclosed by the loop. Special handling is needed for 'zigzag'
    // pipes like L7 and FJ (optionally including horizontal pipes) which function as a vertical pipe.
    // But pipes like LJ and F7 are 'U-turns' which don't change the state of the loop.
    // Credits to @MPinna for the idea!
    for y in 0..height {
        let mut currently_enclosed = false;
        let mut last_curve_shape: Option<Shape> = None;
        for x in 0..width {
            let pos = Position { x, y };
            let tile = grid.get(&pos).unwrap();
            match tile {
                Tile::Ground => count_enclosed += currently_enclosed as i32,
                Tile::Pipe {
                    pos: _,
                    shape: current_shape,
                } => {
                    if !main_loop.contains(&pos) {
                        // junk pipe
                        count_enclosed += currently_enclosed as i32;
                        continue;
                    }
                    if current_shape == &Shape::Horizontal {
                        // Continuing a curve
                        continue;
                    }
                    if current_shape == &Shape::Vertical {
                        // flip the enclosure
                        last_curve_shape = None;
                        currently_enclosed = !currently_enclosed;
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

                    if match (&last_curve, current_shape) {
                        (Shape::NorthEast, Shape::SouthWest) => true,
                        (Shape::SouthEast, Shape::NorthWest) => true,
                        _ => false,
                    } {
                        // L7, L---7, FJ, F---J, etc are zigzags that function the same as a |, flip the enclosure
                        last_curve_shape = None;
                        currently_enclosed = !currently_enclosed;
                        continue;
                    }

                    last_curve_shape = None;
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
