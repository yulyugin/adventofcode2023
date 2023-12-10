use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("Task1 answer: {}", task1::handle_input(&input));
    // println!("Task2 answer: {}", task2::handle_input(&input));

    Ok(())
}

mod task1 {
    type Map = Vec<Vec<char>>;

    fn parse_map(input: &str) -> Map {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    #[test]
    fn test_parse_map() {
        assert_eq!(
            parse_map(
                ".....
.F-7.
.|.|.
.L-J.
....."
            ),
            vec![
                vec!['.'; 5],
                vec!['.', 'F', '-', '7', '.'],
                vec!['.', '|', '.', '|', '.'],
                vec!['.', 'L', '-', 'J', '.'],
                vec!['.'; 5],
            ]
        );
    }

    fn find_start(map: &Map) -> (usize, usize) {
        for (row_index, row) in map.iter().enumerate() {
            for (column_index, elem) in row.iter().enumerate() {
                if elem == &'S' {
                    return (row_index, column_index);
                }
            }
        }
        unreachable!();
    }

    #[test]
    fn test_find_start() {
        assert_eq!(
            find_start(&parse_map(
                ".....
.S-7.
.|.|.
.L-J.
....."
            )),
            (1, 1)
        );
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Direction {
        Left,
        Right,
        Up,
        Down,
        End,
    }

    fn map_get(map: &Map, (x, y): (usize, usize), direction: &Direction) -> char {
        let x = match direction {
            Direction::Up if x > 0 => x - 1,
            Direction::Down => x + 1,
            _ => x,
        };
        let y = match direction {
            Direction::Left if y > 0 => y - 1,
            Direction::Right => y + 1,
            _ => y,
        };
        match map.get(x) {
            Some(row) => match row.get(y) {
                Some(c) => *c,
                None => 'O',
            },
            None => 'O',
        }
    }

    fn start_direction((x, y): (usize, usize), map: &Map) -> Direction {
        assert!(map[x][y] == 'S');
        let left = map_get(&map, (x, y), &Direction::Left);
        let right = map_get(&map, (x, y), &Direction::Right);
        let up = map_get(&map, (x, y), &Direction::Up);
        let down = map_get(&map, (x, y), &Direction::Down);
        if vec!['-', 'L', 'F'].contains(&left) {
            return Direction::Left;
        } else if vec!['|', 'F', '7'].contains(&up) {
            return Direction::Up;
        } else if vec!['-', '7', 'J'].contains(&right) {
            return Direction::Right;
        } else if vec!['|', 'L', 'J'].contains(&down) {
            return Direction::Down;
        }
        unreachable!();
    }

    #[test]
    fn test_start_direction() {
        let map = parse_map(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        let start = find_start(&map);
        assert_eq!(start_direction(start, &map), Direction::Right);

        let map = parse_map(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        let start = find_start(&map);
        assert_eq!(start_direction(start, &map), Direction::Right);
    }

    fn next((x, y): &mut (usize, usize), map: &Map, direction: &mut Direction) {
        *x = match direction {
            Direction::Up => *x - 1,
            Direction::Down => *x + 1,
            _ => *x,
        };
        *y = match direction {
            Direction::Left => *y - 1,
            Direction::Right => *y + 1,
            _ => *y,
        };

        let next = map[*x][*y];
        *direction = match direction {
            Direction::Left => match next {
                '-' => *direction,
                'F' => Direction::Down,
                'L' => Direction::Up,
                'S' => Direction::End,
                _ => unreachable!(),
            },
            Direction::Right => match next {
                '-' => *direction,
                '7' => Direction::Down,
                'J' => Direction::Up,
                'S' => Direction::End,
                _ => unreachable!(),
            },
            Direction::Up => match next {
                '|' => *direction,
                'F' => Direction::Right,
                '7' => Direction::Left,
                'S' => Direction::End,
                _ => unreachable!(),
            },
            Direction::Down => match next {
                '|' => *direction,
                'L' => Direction::Right,
                'J' => Direction::Left,
                'S' => Direction::End,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_next() {
        let map = parse_map(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        let mut point = find_start(&map);
        let mut direction = start_direction(point, &map);

        next(&mut point, &map, &mut direction);
        assert_eq!(point, (1, 2));
        assert_eq!(direction, Direction::Right);

        next(&mut point, &map, &mut direction);
        assert_eq!(point, (1, 3));
        assert_eq!(direction, Direction::Down);

        next(&mut point, &map, &mut direction);
        assert_eq!(point, (2, 3));
        assert_eq!(direction, Direction::Down);

        next(&mut point, &map, &mut direction);
        assert_eq!(point, (3, 3));
        assert_eq!(direction, Direction::Left);

        next(&mut point, &map, &mut direction);
        assert_eq!(point, (3, 2));
        assert_eq!(direction, Direction::Left);

        next(&mut point, &map, &mut direction);
        assert_eq!(point, (3, 1));
        assert_eq!(direction, Direction::Up);

        next(&mut point, &map, &mut direction);
        assert_eq!(point, (2, 1));
        assert_eq!(direction, Direction::Up);

        next(&mut point, &map, &mut direction);
        assert_eq!(point, (1, 1));
        assert_eq!(direction, Direction::End);
    }

    fn loop_length(input: &str) -> usize {
        let map = parse_map(input);
        let mut point = find_start(&map);
        let mut direction = start_direction(point, &map);
        let mut lenght = 0;
        while direction != Direction::End {
            next(&mut point, &map, &mut direction);
            lenght += 1;
        }
        lenght
    }

    #[test]
    fn test_loop_length() {
        assert_eq!(
            loop_length(
                ".....
.S-7.
.|.|.
.L-J.
....."
            ),
            8
        );

        assert_eq!(
            loop_length(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            16
        );

        assert_eq!(
            loop_length(
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"
            ),
            16
        );
    }

    pub fn handle_input(input: &str) -> usize {
        loop_length(input) / 2
    }
}
