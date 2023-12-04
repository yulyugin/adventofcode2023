use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let t1_sum = task1::handle_input(&input);
    println!("Task1 answer: {t1_sum}");

    Ok(())
}

mod task1 {
    pub fn handle_input(input: &str) -> u32 {
        input.lines().map(|l| line_value(l)).sum()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }

    fn line_value(line: &str) -> u32 {
        let matches = matches(line);
        if matches != 0 {
            return u32::pow(2, matches - 1);
        }
        0
    }

    #[test]
    fn test_line_values() {
        assert_eq!(
            line_value("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );
        assert_eq!(
            line_value(
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            "
            ),
            2
        );
        assert_eq!(
            line_value("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            line_value("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            line_value("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            line_value("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }

    fn matches(line: &str) -> u32 {
        let (winning_numbers, numbers_you_have) =
            line.split_once(":").unwrap().1.split_once("|").unwrap();
        return intersect(str_to_list(winning_numbers), str_to_list(numbers_you_have)).len() as u32;
    }

    #[test]
    fn test_matches() {
        assert_eq!(
            matches("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            4
        );
        assert_eq!(
            matches(
                "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            "
            ),
            2
        );
        assert_eq!(
            matches("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            matches("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            matches("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            matches("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }

    fn intersect(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
        a.into_iter().filter(|i| b.contains(i)).collect()
    }

    #[test]
    fn test_intersect() {
        assert_eq!(
            intersect(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
            vec![48, 83, 86, 17]
        );
    }

    fn str_to_list(input: &str) -> Vec<u32> {
        input
            .trim()
            .split(" ")
            .filter_map(|i| i.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>()
    }

    #[test]
    fn test_str_to_list() {
        assert_eq!(str_to_list(" 41 48 83 86 17 "), vec![41, 48, 83, 86, 17]);
        assert_eq!(
            str_to_list(" 83 86  6 31 17  9 48 53"),
            vec![83, 86, 6, 31, 17, 9, 48, 53]
        );
    }
}
