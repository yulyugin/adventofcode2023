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
    pub fn handle_input(input: &str) -> usize {
        count_visited(input, Light::new(0, 0, Direction::Right))
    }

    pub fn count_visited(input: &str, start: Light) -> usize {
        let mut map = Map::new(input);
        let mut lights = vec![start];
        while !lights.is_empty() {
            match lights.pop() {
                Some(light) => lights.append(&mut map.visit(&light)),
                None => break,
            }
        }
        map.count_visited()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            46
        );
    }

    struct Map {
        points: Vec<Vec<Point>>,
    }

    impl Map {
        fn new(input: &str) -> Self {
            Self {
                points: input
                    .lines()
                    .enumerate()
                    .map(|(y, l)| {
                        l.chars()
                            .enumerate()
                            .map(|(x, v)| Point::new(x as i32, y as i32, v))
                            .collect()
                    })
                    .collect(),
            }
        }

        fn get_mut(&mut self, light: &Light) -> Option<&mut Point> {
            if light.x < 0 || light.y < 0 {
                return None;
            }
            self.points
                .get_mut(light.y as usize)?
                .get_mut(light.x as usize)
        }

        pub fn visit(&mut self, light: &Light) -> Vec<Light> {
            match self.get_mut(light) {
                Some(point) => {
                    if point.directions.contains(&light.direction) {
                        return vec![];
                    }
                    point.directions.push(light.direction);
                    point.next(light.direction)
                }
                None => vec![],
            }
        }

        pub fn count_visited(&self) -> usize {
            self.points
                .iter()
                .map(|l| l.iter().filter(|p| p.visited()).count())
                .sum()
        }
    }

    #[derive(Clone, Copy)]
    pub struct Light {
        x: i32,
        y: i32,
        direction: Direction,
    }

    impl Light {
        pub fn new(x: i32, y: i32, direction: Direction) -> Self {
            Self { x, y, direction }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    struct Point {
        x: i32,
        y: i32,
        value: char,
        directions: Vec<Direction>,
    }

    impl Point {
        fn new(x: i32, y: i32, value: char) -> Self {
            Self {
                x,
                y,
                value,
                directions: vec![],
            }
        }

        fn visited(&self) -> bool {
            !self.directions.is_empty()
        }

        fn next(&self, direction: Direction) -> Vec<Light> {
            match self.value {
                '.' => {
                    let (x, y) = match direction {
                        Direction::Right => (self.x + 1, self.y),
                        Direction::Left => (self.x - 1, self.y),
                        Direction::Down => (self.x, self.y + 1),
                        Direction::Up => (self.x, self.y - 1),
                    };
                    vec![Light { x, y, direction }]
                }
                '-' => match direction {
                    Direction::Down | Direction::Up => {
                        vec![
                            Light::new(self.x - 1, self.y, Direction::Left),
                            Light::new(self.x + 1, self.y, Direction::Right),
                        ]
                    }
                    Direction::Right => vec![Light::new(self.x + 1, self.y, Direction::Right)],
                    Direction::Left => vec![Light::new(self.x - 1, self.y, Direction::Left)],
                },
                '|' => match direction {
                    Direction::Left | Direction::Right => {
                        vec![
                            Light::new(self.x, self.y - 1, Direction::Up),
                            Light::new(self.x, self.y + 1, Direction::Down),
                        ]
                    }
                    Direction::Up => vec![Light::new(self.x, self.y - 1, Direction::Up)],
                    Direction::Down => vec![Light::new(self.x, self.y + 1, Direction::Down)],
                },
                '/' => match direction {
                    Direction::Left => vec![Light::new(self.x, self.y + 1, Direction::Down)],
                    Direction::Right => vec![Light::new(self.x, self.y - 1, Direction::Up)],
                    Direction::Up => vec![Light::new(self.x + 1, self.y, Direction::Right)],
                    Direction::Down => vec![Light::new(self.x - 1, self.y, Direction::Left)],
                },
                '\\' => match direction {
                    Direction::Left => vec![Light::new(self.x, self.y - 1, Direction::Up)],
                    Direction::Right => vec![Light::new(self.x, self.y + 1, Direction::Down)],
                    Direction::Down => vec![Light::new(self.x + 1, self.y, Direction::Right)],
                    Direction::Up => vec![Light::new(self.x - 1, self.y, Direction::Left)],
                },
                _ => unreachable!(),
            }
        }
    }
}

mod task2 {
    use crate::task1::{count_visited, Direction, Light};

    pub fn handle_input(input: &str) -> usize {
        let y_len = input.lines().count() as i32;
        let x_len = input.lines().nth(0).unwrap().chars().count() as i32;
        let mut starts = vec![];
        for x in 0..x_len {
            starts.push(Light::new(x, 0, Direction::Down));
            starts.push(Light::new(x, y_len - 1, Direction::Up));
        }
        for y in 0..y_len {
            starts.push(Light::new(0, y, Direction::Right));
            starts.push(Light::new(x_len - 1, y, Direction::Left));
        }
        starts
            .iter()
            .map(|s| count_visited(input, *s))
            .max()
            .unwrap()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            ),
            51
        );
    }
}
