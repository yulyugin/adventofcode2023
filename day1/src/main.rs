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

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let sum = task1::handle_input(&input);
    println!("Task1 answer: {sum}");
    Ok(())
}
