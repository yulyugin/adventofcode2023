use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let t1_sum = task1::handle_input(&input);
    println!("Task1 answer: {t1_sum}");

    let t2_sum = task2::handle_input(&input);
    println!("Task1 answer: {t2_sum}");

    Ok(())
}

mod task1 {
    use regex::Match;

    #[derive(Debug, PartialEq)]
    pub struct Number {
        pub value: u32,
        start: usize,
        end: usize,
    }

    impl Number {
        fn new(value: u32, start: usize, end: usize) -> Self {
            Self { value, start, end }
        }

        fn from_match(m: Match<'_>) -> Self {
            let mut start = m.start();
            if start != 0 {
                start -= 1;
            }
            Self {
                value: m.as_str().parse::<u32>().unwrap(),
                start,
                end: m.end(),
            }
        }

        fn is_adjacent(&self, symbol: &Symbol) -> bool {
            if (self.start <= symbol.position) && (symbol.position <= self.end) {
                return true;
            }
            false
        }
    }

    #[test]
    fn test_is_adjacent() {
        assert!(Number::new(467, 0, 3).is_adjacent(&Symbol::new(3)));
        assert!(!Number::new(114, 4, 8).is_adjacent(&Symbol::new(3)));
    }

    pub fn find_numbers(line: &str) -> Vec<Number> {
        let mut results = vec![];
        let re = regex::Regex::new(r"\d+").unwrap();
        for number in re.find_iter(line) {
            results.push(Number::from_match(number));
        }
        results
    }

    #[test]
    fn test_find_numbers() {
        assert_eq!(
            find_numbers("467..114.."),
            vec![Number::new(467, 0, 3), Number::new(114, 4, 8)]
        );
        assert_eq!(find_numbers("...*......"), vec![]);
        assert_eq!(
            find_numbers("..35..633."),
            vec![Number::new(35, 1, 4), Number::new(633, 5, 9)]
        );
        assert_eq!(find_numbers("......#..."), vec![]);
        assert_eq!(find_numbers("617*......"), vec![Number::new(617, 0, 3)]);
        assert_eq!(find_numbers(".....+.58."), vec![Number::new(58, 6, 9)]);
        assert_eq!(find_numbers("..592....."), vec![Number::new(592, 1, 5)]);
        assert_eq!(find_numbers("......755."), vec![Number::new(755, 5, 9)]);
        assert_eq!(find_numbers("...$.*...."), vec![]);
        assert_eq!(
            find_numbers(".664.598.."),
            vec![Number::new(664, 0, 4), Number::new(598, 4, 8)]
        );
    }

    #[derive(Debug, PartialEq)]
    pub struct Symbol {
        position: usize,
    }

    impl Symbol {
        pub fn new(position: usize) -> Self {
            Self { position }
        }

        pub fn find_adjacent_numbers(&self, prev: &Line, this: &Line, next: &Line) -> Vec<u32> {
            let mut results = vec![];
            for n in &prev.numbers {
                if n.is_adjacent(self) {
                    results.push(n.value);
                }
            }
            for n in &this.numbers {
                if n.is_adjacent(self) {
                    results.push(n.value);
                }
            }
            for n in &next.numbers {
                if n.is_adjacent(self) {
                    results.push(n.value);
                }
            }
            results
        }
    }

    fn find_symbols(line: &str) -> Vec<Symbol> {
        let mut results = vec![];
        for (i, s) in line.chars().enumerate() {
            if !s.is_ascii_digit() && !(s == '.') {
                results.push(Symbol::new(i));
            }
        }
        results
    }

    #[test]
    fn test_find_symbols() {
        assert_eq!(find_symbols("467..114.."), vec![]);
        assert_eq!(find_symbols("...*......"), vec![Symbol::new(3)]);
        assert_eq!(find_symbols("..35..633."), vec![]);
        assert_eq!(find_symbols("......#..."), vec![Symbol::new(6)]);
        assert_eq!(find_symbols("617*......"), vec![Symbol::new(3)]);
        assert_eq!(find_symbols(".....+.58."), vec![Symbol::new(5)]);
        assert_eq!(find_symbols("..592....."), vec![]);
        assert_eq!(find_symbols("......755."), vec![]);
        assert_eq!(
            find_symbols("...$.*...."),
            vec![Symbol::new(3), Symbol::new(5)]
        );
        assert_eq!(find_symbols(".664.598.."), vec![]);
    }

    pub struct Line {
        pub symbols: Vec<Symbol>,
        pub numbers: Vec<Number>,
    }

    impl Line {
        pub fn empty() -> Self {
            Self {
                symbols: vec![],
                numbers: vec![],
            }
        }

        pub fn new(line: &str) -> Self {
            Self {
                symbols: find_symbols(line),
                numbers: find_numbers(line),
            }
        }

        fn find_adjacent_numbers(&self, prev: &Line, next: &Line) -> Vec<u32> {
            let mut results = vec![];
            for s in &self.symbols {
                results.append(&mut s.find_adjacent_numbers(prev, self, next));
            }
            results
        }
    }

    fn find_adjacent_numbers(input: &str) -> Vec<u32> {
        let mut results = vec![];

        let mut this = Line::empty();
        let mut prev = Line::empty();
        let mut next = Line::empty();

        for l in input.lines() {
            prev = this;
            this = next;
            next = Line::new(l);

            results.append(&mut this.find_adjacent_numbers(&prev, &next));
        }
        results.append(&mut next.find_adjacent_numbers(&this, &Line::empty()));
        results
    }

    #[test]
    fn test_find_adjacent_numbers() {
        assert_eq!(
            find_adjacent_numbers(
                "467..114..
...*......"
            ),
            vec![467]
        );
        assert_eq!(
            find_adjacent_numbers(
                "467..114..
...*......
..35..633."
            ),
            vec![467, 35]
        );
        assert_eq!(
            find_adjacent_numbers(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            vec![467, 35, 633, 617, 592, 664, 755, 598]
        );
    }

    pub fn handle_input(input: &str) -> u32 {
        find_adjacent_numbers(input).iter().sum()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        );
    }
}

mod task2 {
    use crate::task1::{find_numbers, Line, Symbol};

    fn find_gear_symbols(line: &str) -> Vec<Symbol> {
        let mut results = vec![];
        for (i, s) in line.chars().enumerate() {
            if s == '*' {
                results.push(Symbol::new(i));
            }
        }
        results
    }

    #[test]
    fn test_find_gear_symbols() {
        assert_eq!(find_gear_symbols("467..114.."), vec![]);
        assert_eq!(find_gear_symbols("...*......"), vec![Symbol::new(3)]);
        assert_eq!(find_gear_symbols("..35..633."), vec![]);
        assert_eq!(find_gear_symbols("......#..."), vec![]);
        assert_eq!(find_gear_symbols("617*......"), vec![Symbol::new(3)]);
        assert_eq!(find_gear_symbols(".....+.58."), vec![]);
        assert_eq!(find_gear_symbols("..592....."), vec![]);
        assert_eq!(find_gear_symbols("......755."), vec![]);
        assert_eq!(find_gear_symbols("...$.*...."), vec![Symbol::new(5)]);
        assert_eq!(find_gear_symbols(".664.598.."), vec![]);
    }

    impl Line {
        fn find_gears(&self, prev: &Line, next: &Line) -> Vec<u32> {
            let mut results = vec![];
            for s in &self.symbols {
                let adjacent_numbers = s.find_adjacent_numbers(prev, self, next);
                if adjacent_numbers.len() == 2 {
                    results.push(adjacent_numbers[0] * adjacent_numbers[1]);
                }
            }
            results
        }

        fn new_gears(line: &str) -> Self {
            Self {
                symbols: find_gear_symbols(line),
                numbers: find_numbers(line),
            }
        }
    }

    fn find_gears(input: &str) -> Vec<u32> {
        let mut results = vec![];

        let mut this = Line::empty();
        let mut prev = Line::empty();
        let mut next = Line::empty();

        for l in input.lines() {
            prev = this;
            this = next;
            next = Line::new_gears(l);

            results.append(&mut this.find_gears(&prev, &next));
        }
        results.append(&mut next.find_gears(&this, &Line::empty()));
        results
    }

    #[test]
    fn test_find_gears() {
        assert_eq!(
            find_gears(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            vec![16345, 451490]
        );
    }

    pub fn handle_input(input: &str) -> u32 {
        find_gears(input).iter().sum()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            467835
        );
    }
}
