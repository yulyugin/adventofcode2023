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
    use std::cmp::min;

    type Map = Vec<Vec<char>>;

    pub fn handle_input(input: &str) -> usize {
        let mut result = 0;
        for pattern in input.split("\n\n") {
            let map = read_map(pattern);

            result += column_reflection_cut(&map);
            result += 100 * row_reflection_cut(&map);
        }
        result
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            ),
            405
        );
    }

    #[cfg(test)]
    fn test_map1() -> Map {
        read_map(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        )
    }

    #[cfg(test)]
    fn test_map2() -> Map {
        read_map(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        )
    }

    #[cfg(test)]
    fn test_map3() -> Map {
        read_map(
            "#...##...#..#.#..
#...##...#..#.#..
.#.#...#.#....#..
#.#..#.##..##.#.#",
        )
    }

    fn read_map(map: &str) -> Map {
        map.lines().map(|l| l.chars().collect()).collect()
    }

    fn transpose(map: &Map) -> Map {
        let mut transposed = vec![];
        for c in 0..map[0].len() {
            let mut column = vec![];
            for r in 0..map.len() {
                column.push(map[r][c]);
            }
            transposed.push(column);
        }
        transposed
    }

    fn column_reflection_cut(map: &Map) -> usize {
        let map = transpose(map);
        row_reflection_cut(&map)
    }

    #[test]
    fn test_column_reflection() {
        assert_eq!(column_reflection_cut(&test_map1()), 5);
        assert_eq!(column_reflection_cut(&test_map2()), 0);
    }

    fn is_reflection_row(map: &Map, row: usize) -> bool {
        let mirror_row = row + 1;
        if mirror_row == map.len() {
            return false;
        }
        for i in 0..min(row - 0, map.len() - 1 - mirror_row) + 1 {
            if map[row - i] != map[mirror_row + i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_is_reflection_row() {
        let map = test_map1();
        assert!(!is_reflection_row(&map, 0));
        assert!(!is_reflection_row(&map, 1));
        assert!(!is_reflection_row(&map, 2));
        assert!(!is_reflection_row(&map, 3));

        let map = test_map2();
        assert!(!is_reflection_row(&map, 1));
        assert!(!is_reflection_row(&map, 2));
        assert!(is_reflection_row(&map, 3));
        assert!(!is_reflection_row(&map, 4));

        let map = test_map3();
        assert!(is_reflection_row(&map, 0));
        assert!(!is_reflection_row(&map, 1));
    }

    fn row_reflection_cut(map: &Map) -> usize {
        let matching_rows: Vec<usize> = map
            .windows(2)
            .enumerate()
            .filter_map(|(i, a)| if a[0] == a[1] { Some(i) } else { None })
            .collect();
        for row in matching_rows {
            if is_reflection_row(&map, row) {
                return row + 1;
            }
        }
        0
    }

    #[test]
    fn test_row_reflection() {
        assert_eq!(row_reflection_cut(&test_map1()), 0);
        assert_eq!(row_reflection_cut(&test_map2()), 4);
        assert_eq!(row_reflection_cut(&test_map3()), 1);
    }
}
