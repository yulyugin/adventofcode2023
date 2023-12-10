use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("Task1 answer: {}", task1::handle_input(&input));
    println!("Task2 answer: {}", task2::handle_input(&input));

    Ok(())
}

mod task1 {
    pub type Map = Vec<Vec<char>>;

    pub fn parse_map(input: &str) -> Map {
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

    pub fn find_start(map: &Map) -> (usize, usize) {
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
    pub enum Direction {
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

    pub fn start_direction((x, y): (usize, usize), map: &Map) -> Direction {
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

    pub fn next((x, y): &mut (usize, usize), map: &Map, direction: &mut Direction) {
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

mod task2 {
    use crate::task1::{find_start, next, parse_map, start_direction, Direction, Map};

    type PointMap = Vec<Vec<Point>>;

    #[derive(Clone, Debug, PartialEq)]
    enum PointClass {
        Loop,
        Outside,
        None,
    }

    #[derive(Clone, Debug)]
    struct Point {
        point: (usize, usize),
        class: PointClass,
        value: char,
    }

    fn create_point_map(map: &Map) -> PointMap {
        let mut point_map = vec![vec![]; map.len()];
        for (x, row) in map.iter().enumerate() {
            let row_points = &mut point_map[x];
            for y in 0..row.len() {
                row_points.push(Point {
                    point: (x, y),
                    class: PointClass::None,
                    value: map[x][y],
                });
            }
        }
        point_map
    }

    pub fn handle_input(input: &str) -> u32 {
        let map = parse_map(input);
        count_inner_points(&map)
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                ".....
.S-7.
.|.|.
.L-J.
....."
            ),
            1
        );

        assert_eq!(
            handle_input(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            ),
            1
        );

        assert_eq!(
            handle_input(
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

        assert_eq!(
            handle_input(
                "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."
            ),
            4
        );

        assert_eq!(
            handle_input(
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

        assert_eq!(
            handle_input(
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

    fn count_inner_points(map: &Map) -> u32 {
        let mut point_map = create_point_map(map);
        mark_loop(&mut point_map, map);
        mark_outside_points(&mut point_map);
        let mut inner_points = 0;
        for row in point_map {
            for point in row {
                if point.class == PointClass::None {
                    inner_points += 1;
                }
            }
        }
        inner_points
    }

    fn mark_surface_points(point_map: &mut PointMap) {
        for row in point_map.iter_mut() {
            for point in row.iter_mut() {
                if point.class == PointClass::None {
                    point.class = PointClass::Outside;
                } else if point.class == PointClass::Loop {
                    break;
                }
            }
            for point in row.iter_mut().rev() {
                if point.class == PointClass::None {
                    point.class = PointClass::Outside;
                } else if point.class == PointClass::Loop {
                    break;
                }
            }
        }
    }

    enum Surface {
        Left,
        Right,
        Up,
        Down,
    }

    fn update_surface(point_map: &mut PointMap, (x, y): (&usize, &usize), surface: &Surface) {
        let point = &point_map[*x][*y];
        assert_eq!(point.class, PointClass::Loop);
        let (x, y) = point.point;
        let x = x as i32;
        let y = y as i32;
        let points_to_update = match point.value {
            '|' => match surface {
                Surface::Left => vec![(x, y - 1)],
                Surface::Right => vec![(x, y + 1)],
                _ => unreachable!(),
            },
            '-' => match surface {
                Surface::Down => vec![(x + 1, y)],
                Surface::Up => vec![(x - 1, y)],
                _ => unreachable!(),
            },
            'F' => match surface {
                Surface::Left | Surface::Up => vec![(x, y - 1), (x - 1, y - 1), (x - 1, y)],
                Surface::Down | Surface::Right => vec![(x + 1, y + 1)],
            },
            '7' => match surface {
                Surface::Right | Surface::Up => vec![(x - 1, y), (x - 1, y + 1), (x, y + 1)],
                Surface::Left | Surface::Down => vec![(x + 1, y - 1)],
            },
            'J' => match surface {
                Surface::Down | Surface::Right => vec![(x, y + 1), (x + 1, y + 1), (x + 1, y)],
                Surface::Left | Surface::Up => vec![(x - 1, y - 1)],
            },
            'L' => match surface {
                Surface::Left | Surface::Down => vec![(x, y - 1), (x + 1, y - 1), (x + 1, y)],
                Surface::Right | Surface::Up => vec![(x - 1, y + 1)],
            },
            'S' => {
                vec![]
            }
            _ => unreachable!(),
        };

        for (x, y) in points_to_update {
            mark_point_outside(point_map, (x, y));
        }
    }

    fn mark_loop_surface_points(point_map: &mut PointMap) {
        let mut first_loop_point = (0, 0);
        'outer: for row in point_map.iter_mut() {
            for point in row.iter_mut() {
                assert!(point.class != PointClass::None);
                if point.class == PointClass::Loop {
                    first_loop_point = point.point;
                    break 'outer;
                }
            }
        }

        let mut surface = Surface::Up;
        let mut direction = Direction::Right;
        let mut point = first_loop_point;
        while direction != Direction::End {
            surface_step(point_map, &mut point, &mut direction, &mut surface);
        }

        let mut surface = Surface::Left;
        let mut direction = Direction::Down;
        let mut point = first_loop_point;
        while direction != Direction::End {
            surface_step(point_map, &mut point, &mut direction, &mut surface);
        }
    }

    fn surface_step(
        point_map: &mut PointMap,
        (x, y): &mut (usize, usize),
        direction: &mut Direction,
        surface: &mut Surface,
    ) {
        update_surface(point_map, (x, y), surface);
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

        let next = &point_map[*x][*y];
        match direction {
            Direction::Left => match next.value {
                '-' => {}
                'F' => {
                    *direction = Direction::Down;
                    *surface = match surface {
                        Surface::Down => Surface::Right,
                        Surface::Up => Surface::Left,
                        _ => unreachable!(),
                    }
                }
                'L' => {
                    *direction = Direction::Up;
                    *surface = match surface {
                        Surface::Down => Surface::Left,
                        Surface::Up => Surface::Right,
                        _ => unreachable!(),
                    }
                }
                'S' => *direction = Direction::End,
                _ => unreachable!(),
            },
            Direction::Right => match next.value {
                '-' => {}
                '7' => {
                    *direction = Direction::Down;
                    *surface = match surface {
                        Surface::Down => Surface::Left,
                        Surface::Up => Surface::Right,
                        _ => unreachable!(),
                    }
                }
                'J' => {
                    *direction = Direction::Up;
                    *surface = match surface {
                        Surface::Down => Surface::Right,
                        Surface::Up => Surface::Left,
                        _ => unreachable!(),
                    }
                }
                'S' => *direction = Direction::End,
                _ => unreachable!(),
            },
            Direction::Up => match next.value {
                '|' => {}
                'F' => {
                    *direction = Direction::Right;
                    *surface = match surface {
                        Surface::Left => Surface::Up,
                        Surface::Right => Surface::Down,
                        _ => unreachable!(),
                    }
                }
                '7' => {
                    *direction = Direction::Left;
                    *surface = match surface {
                        Surface::Left => Surface::Down,
                        Surface::Right => Surface::Up,
                        _ => unreachable!(),
                    }
                }
                'S' => *direction = Direction::End,
                _ => unreachable!(),
            },
            Direction::Down => match next.value {
                '|' => {}
                'L' => {
                    *direction = Direction::Right;
                    *surface = match surface {
                        Surface::Left => Surface::Down,
                        Surface::Right => Surface::Up,
                        _ => unreachable!(),
                    }
                }
                'J' => {
                    *direction = Direction::Left;
                    *surface = match surface {
                        Surface::Left => Surface::Up,
                        Surface::Right => Surface::Down,
                        _ => unreachable!(),
                    }
                }
                'S' => *direction = Direction::End,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
    }

    fn mark_point_outside(point_map: &mut PointMap, (x, y): (i32, i32)) {
        if x < 0 || y < 0 {
            return;
        }

        match point_map.get_mut(x as usize) {
            Some(row) => match row.get_mut(y as usize) {
                Some(point) => {
                    if point.class == PointClass::None {
                        point.class = PointClass::Outside;
                    }
                }
                None => {}
            },
            None => {}
        };
    }

    fn update_neighbor_points(point_map: &mut PointMap, (x, y): (usize, usize)) {
        if point_map[x][y].class != PointClass::Outside {
            return;
        }
        let x = x as i32;
        let y = y as i32;
        for x in x - 1..x + 2 {
            for y in y - 1..y + 2 {
                mark_point_outside(point_map, (x, y));
            }
        }
    }

    fn mark_neighbor_points(point_map: &mut PointMap) {
        for r in 0..point_map.len() {
            for c in 0..point_map[r].len() {
                update_neighbor_points(point_map, (r, c));
            }
        }
    }

    fn mark_outside_points(point_map: &mut PointMap) {
        mark_surface_points(point_map);
        mark_loop_surface_points(point_map);
        mark_neighbor_points(point_map);
    }

    #[test]
    fn test_mark_outside_points() {
        let map = parse_map(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        let mut point_map = create_point_map(&map);
        mark_loop(&mut point_map, &map);
        mark_outside_points(&mut point_map);
        assert_eq!(point_map[0][0].class, PointClass::Outside);
        assert_eq!(point_map[1][0].class, PointClass::Outside);
        assert_eq!(point_map[1][4].class, PointClass::Outside);
        assert_eq!(point_map[4][3].class, PointClass::Outside);
        assert_eq!(point_map[2][2].class, PointClass::None);
        assert_eq!(point_map[2][3].class, PointClass::Loop);

        let map = parse_map(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        let mut point_map = create_point_map(&map);
        mark_loop(&mut point_map, &map);
        mark_outside_points(&mut point_map);
        assert_eq!(point_map[3][3].class, PointClass::Outside);
        assert_eq!(point_map[4][5].class, PointClass::Outside);
        assert_eq!(point_map[5][5].class, PointClass::Outside);
        assert_eq!(point_map[6][5].class, PointClass::Outside);
    }

    fn mark_loop(point_map: &mut PointMap, map: &Map) {
        let mut point = find_start(&map);
        let start_point = point_map
            .get_mut(point.0)
            .unwrap()
            .get_mut(point.1)
            .unwrap();
        start_point.class = PointClass::Loop;
        let mut direction = start_direction(point, &map);
        while direction != Direction::End {
            next(&mut point, &map, &mut direction);
            let next_point = point_map
                .get_mut(point.0)
                .unwrap()
                .get_mut(point.1)
                .unwrap();
            next_point.class = PointClass::Loop;
        }
    }

    #[test]
    fn test_mark_loop() {
        let map = parse_map(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        let mut point_map = create_point_map(&map);
        mark_loop(&mut point_map, &map);
        assert_eq!(point_map[0][0].class, PointClass::None);
        assert_eq!(point_map[1][1].class, PointClass::Loop);
        assert_eq!(point_map[1][2].class, PointClass::Loop);
        assert_eq!(point_map[1][3].class, PointClass::Loop);
        assert_eq!(point_map[1][4].class, PointClass::None);
        assert_eq!(point_map[2][2].class, PointClass::None);
        assert_eq!(point_map[2][3].class, PointClass::Loop);
    }
}
