use std::fs::File;
use std::io::prelude::*;

mod task1 {
    fn first_digit(str: &str) -> u32 {
        for c in str.chars() {
            if c.is_ascii_digit() {
                return c.to_digit(10).unwrap();
            }
        }
        0
    }

    fn last_digit(str: &str) -> u32 {
        for c in str.chars().rev() {
            if c.is_ascii_digit() {
                return c.to_digit(10).unwrap();
            }
        }
        0
    }

    fn process_line(str: &str) -> u32 {
        first_digit(str) * 10 + last_digit(str)
    }

    pub fn handle_input(str: &str) -> u32 {
        let mut ret = 0;
        for l in str.lines() {
            ret += process_line(l);
        }
        ret
    }

    #[test]
    fn test_first_digit() {
        assert_eq!(first_digit("1abc2"), 1);
        assert_eq!(first_digit("pqr3stu8vwx"), 3);
        assert_eq!(first_digit("a1b2c3d4e5f"), 1);
        assert_eq!(first_digit("treb7uchet"), 7);
    }

    #[test]
    fn test_last_digit() {
        assert_eq!(last_digit("1abc2"), 2);
        assert_eq!(last_digit("pqr3stu8vwx"), 8);
        assert_eq!(last_digit("a1b2c3d4e5f"), 5);
        assert_eq!(last_digit("treb7uchet"), 7);
    }

    #[test]
    fn test_process_line() {
        assert_eq!(process_line("1abc2"), 12);
        assert_eq!(process_line("pqr3stu8vwx"), 38);
        assert_eq!(process_line("a1b2c3d4e5f"), 15);
        assert_eq!(process_line("treb7uchet"), 77);
    }

    #[test]
    fn test_sample_input() {
        assert_eq!(
            handle_input(
                "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            ),
            142
        );
    }
}

mod task2 {
    use std::collections::HashMap;

    fn string_to_digit(str: &str) -> u32 {
        let translation_table = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);
        translation_table[str]
    }

    fn first_digit(str: &str) -> u32 {
        let mut digit: Option<(usize, u32)> = None;
        for (i, c) in str.chars().enumerate() {
            if c.is_ascii_digit() {
                digit = Some((i, c.to_digit(10).unwrap()));
                break;
            }
        }

        let re =
            regex::Regex::new("(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
                .unwrap();
        match re.find(str) {
            Some(m) => match digit {
                Some((di, dval)) if di < m.start() => dval,
                _ => string_to_digit(m.as_str()),
            },
            None => match digit {
                Some((_, value)) => value,
                None => panic!(),
            },
        }
    }

    fn last_digit(str: &str) -> u32 {
        let mut digit: Option<(usize, u32)> = None;
        for (i, c) in str.chars().rev().enumerate() {
            if c.is_ascii_digit() {
                digit = Some((str.len() - i - 1, c.to_digit(10).unwrap()));
                break;
            }
        }

        for dstr in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ] {
            let re = regex::Regex::new(dstr).unwrap();
            digit = match re.find_iter(str).last() {
                Some(m) => match digit {
                    Some((di, _)) if di > m.start() => digit,
                    _ => Some((m.start(), string_to_digit(m.as_str()))),
                },
                None => digit,
            }
        }

        match digit {
            Some((_, val)) => val,
            None => panic!(),
        }
    }

    fn process_line(str: &str) -> u32 {
        first_digit(str) * 10 + last_digit(str)
    }

    pub fn handle_input(str: &str) -> u32 {
        let mut ret = 0;
        for l in str.lines() {
            ret += process_line(l);
        }
        ret
    }

    #[test]
    fn test_sample_input() {
        assert_eq!(
            handle_input(
                "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"
            ),
            281
        );
    }

    #[test]
    fn test_process_line() {
        assert_eq!(process_line("two1nine"), 29);
        assert_eq!(process_line("eightwothree"), 83);
        assert_eq!(process_line("abcone2threexyz"), 13);
        assert_eq!(process_line("xtwone3four"), 24);
        assert_eq!(process_line("4nineeightseven2"), 42);
        assert_eq!(process_line("zoneight234"), 14);
        assert_eq!(process_line("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_last_digit() {
        assert_eq!(last_digit("two1nine"), 9);
        assert_eq!(last_digit("eightwothree"), 3);
        assert_eq!(last_digit("abcone2threexyz"), 3);
        assert_eq!(last_digit("xtwone3four"), 4);
        assert_eq!(last_digit("4nineeightseven2"), 2);
        assert_eq!(last_digit("zoneight234"), 4);
        assert_eq!(last_digit("7pqrstsixteen"), 6);
        assert_eq!(last_digit("twone"), 1);
        assert_eq!(last_digit("sevenine"), 9);
    }

    #[test]
    fn test_first_digit() {
        assert_eq!(first_digit("two1nine"), 2);
        assert_eq!(first_digit("eightwothree"), 8);
        assert_eq!(first_digit("abcone2threexyz"), 1);
        assert_eq!(first_digit("xtwone3four"), 2);
        assert_eq!(first_digit("4nineeightseven2"), 4);
        assert_eq!(first_digit("zoneight234"), 1);
        assert_eq!(first_digit("7pqrstsixteen"), 7);
        assert_eq!(first_digit("twone"), 2);
        assert_eq!(first_digit("sevenine"), 7);
    }

    #[test]
    fn test_string_to_digit() {
        assert_eq!(string_to_digit("one"), 1);
        assert_eq!(string_to_digit("two"), 2);
        assert_eq!(string_to_digit("three"), 3);
        assert_eq!(string_to_digit("four"), 4);
        assert_eq!(string_to_digit("five"), 5);
        assert_eq!(string_to_digit("six"), 6);
        assert_eq!(string_to_digit("seven"), 7);
        assert_eq!(string_to_digit("eight"), 8);
        assert_eq!(string_to_digit("nine"), 9);
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let t1_sum = task1::handle_input(&input);
    println!("Task1 answer: {t1_sum}");

    let t2_sum = task2::handle_input(&input);
    println!("Task2 answer: {t2_sum}");

    Ok(())
}
