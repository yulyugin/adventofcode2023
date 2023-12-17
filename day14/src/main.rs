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

    pub fn handle_input(input: &str) -> usize {
        let mut map = read_map(input);
        lean_north(&mut map);
        total_load(&map)
    }

    fn total_load(map: &Map) -> usize {
        map.iter()
            .enumerate()
            .map(|(i, r)| r.iter().filter(|v| **v == 'O').count() * (map.len() - i))
            .sum()
    }

    #[test]
    fn test_total_load() {
        let mut map = read_map(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        lean_north(&mut map);
        assert_eq!(total_load(&map), 136);
    }

    fn read_map(input: &str) -> Map {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    fn lean_north(map: &mut Map) {
        for x in 0..map[0].len() {
            let mut to = 0;
            for y in 0..map.len() {
                match map[y][x] {
                    'O' => {
                        let tmp = map[to][x];
                        map[to][x] = map[y][x];
                        map[y][x] = tmp;
                        to += 1;
                    }
                    '.' => {}
                    '#' => {
                        to = y + 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn test_lean_north() {
        let mut map = read_map(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        lean_north(&mut map);
        assert_eq!(
            read_map(
                "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
            ),
            map
        );
    }
}
