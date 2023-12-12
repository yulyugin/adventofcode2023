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
    pub fn handle_input(input: &str) -> i64 {
        calculate_total_distance(input, 1)
    }

    pub fn calculate_total_distance(input: &str, expansion_coefficient: usize) -> i64 {
        let map = read_map(input);
        let planets = planets_expanded(&map, expansion_coefficient);
        let mut sum = 0;
        for (i, this) in planets.iter().enumerate() {
            for other in &planets[i + 1..] {
                sum += calculate_shortest_distance(this, other);
            }
        }
        sum
    }

    #[test]
    fn test_expansion_coefficient() {
        assert_eq!(
            calculate_total_distance(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                1
            ),
            374
        );

        assert_eq!(
            calculate_total_distance(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                9
            ),
            1030
        );

        assert_eq!(
            calculate_total_distance(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                99
            ),
            8410
        );
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            374
        );
    }

    fn calculate_shortest_distance(this: &Planet, other: &Planet) -> i64 {
        (this.x as i64 - other.x as i64).abs() + (this.y as i64 - other.y as i64).abs()
    }

    #[test]
    fn test_calculate_shortest_distance() {
        let this = Planet { x: 0, y: 4 };
        let other = Planet { x: 10, y: 9 };
        assert_eq!(calculate_shortest_distance(&this, &other), 15);
        assert_eq!(calculate_shortest_distance(&other, &this), 15);

        let this = Planet { x: 11, y: 0 };
        let other = Planet { x: 11, y: 5 };
        assert_eq!(calculate_shortest_distance(&this, &other), 5);
        assert_eq!(calculate_shortest_distance(&other, &this), 5);
    }

    fn planets_expanded(map: &Vec<Vec<char>>, expansion_coefficient: usize) -> Vec<Planet> {
        let mut planets = planets(map);
        for row in empty_rows(map).iter().rev() {
            for p in planets.iter_mut() {
                if p.x > *row {
                    p.x += expansion_coefficient;
                }
            }
        }
        for column in empty_columns(map).iter().rev() {
            for p in planets.iter_mut() {
                if p.y > *column {
                    p.y += expansion_coefficient;
                }
            }
        }
        planets
    }

    #[test]
    fn test_planets_expanded() {
        let map = read_map(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(
            planets_expanded(&map, 1),
            vec![
                Planet { x: 0, y: 4 },
                Planet { x: 1, y: 9 },
                Planet { x: 2, y: 0 },
                Planet { x: 5, y: 8 },
                Planet { x: 6, y: 1 },
                Planet { x: 7, y: 12 },
                Planet { x: 10, y: 9 },
                Planet { x: 11, y: 0 },
                Planet { x: 11, y: 5 },
            ]
        );
    }

    #[derive(Debug, PartialEq)]
    struct Planet {
        x: usize,
        y: usize,
    }

    fn planets(map: &Vec<Vec<char>>) -> Vec<Planet> {
        let mut result = vec![];
        for (x, row) in map.iter().enumerate() {
            for (y, elem) in row.iter().enumerate() {
                if *elem == '#' {
                    result.push(Planet { x, y });
                }
            }
        }
        result
    }

    #[test]
    fn test_planet_detection() {
        let map = read_map(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(
            planets(&map),
            vec![
                Planet { x: 0, y: 3 },
                Planet { x: 1, y: 7 },
                Planet { x: 2, y: 0 },
                Planet { x: 4, y: 6 },
                Planet { x: 5, y: 1 },
                Planet { x: 6, y: 9 },
                Planet { x: 8, y: 7 },
                Planet { x: 9, y: 0 },
                Planet { x: 9, y: 4 },
            ]
        );
    }

    fn read_map(input: &str) -> Vec<Vec<char>> {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    fn empty_columns(map: &Vec<Vec<char>>) -> Vec<usize> {
        let mut result = vec![];
        'outer: for c in 0..map[0].len() {
            for r in 0..map.len() {
                if map[0][c] != map[r][c] {
                    continue 'outer;
                }
            }
            result.push(c);
        }
        result
    }

    #[test]
    fn test_empty_columns() {
        let map = read_map(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(empty_columns(&map), vec![2, 5, 8]);
    }

    fn empty_rows(map: &Vec<Vec<char>>) -> Vec<usize> {
        map.iter()
            .enumerate()
            .filter_map(|(i, r)| {
                if r.windows(2).all(|v| v[0] == v[1]) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn test_empty_rows() {
        let map = read_map(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(empty_rows(&map), vec![3, 7]);
    }
}

mod task2 {
    use crate::task1::calculate_total_distance;

    pub fn handle_input(input: &str) -> i64 {
        calculate_total_distance(input, 1000000 - 1)
    }
}
