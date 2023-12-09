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
    pub fn handle_input(input: &str) -> i32 {
        input.lines().map(|l| extrapolate(l)).sum()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            114
        );
    }

    fn parse_sequence(input: &str) -> Vec<i32> {
        input
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
    }

    #[test]
    fn test_parse_sequence() {
        assert_eq!(parse_sequence("0 3 -6 9 12 15"), vec![0, 3, -6, 9, 12, 15]);
    }

    fn diff_sequence(sequence: &Vec<i32>) -> Vec<i32> {
        let mut results = vec![];
        for i in 1..sequence.len() {
            results.push(sequence[i] - sequence[i - 1]);
        }
        assert!(results.len() == sequence.len() - 1);
        results
    }

    #[test]
    fn test_diff_sequence() {
        assert_eq!(diff_sequence(&vec![0, 3, 6, 9, 12, 15]), vec![3; 5]);
    }

    fn all_zeros(sequence: &Vec<i32>) -> bool {
        sequence.iter().sum::<i32>() == 0
    }

    #[test]
    fn test_all_zeros() {
        assert!(all_zeros(&vec![0; 15]));
        assert!(!all_zeros(&vec![0, 0, 0, 0, 0, 1]));
        assert!(!all_zeros(&vec![3, 0, 2, 0, 0, 0]));
    }

    fn extrapolate(input: &str) -> i32 {
        let start = parse_sequence(input);
        let mut current = &start;
        let mut sequences: Vec<Vec<i32>> = vec![];
        loop {
            let diff = diff_sequence(&current);
            if all_zeros(&diff) {
                break;
            }
            sequences.push(diff);
            current = &sequences[sequences.len() - 1];
        }
        let mut extrapolation = 0;
        for s in sequences {
            extrapolation += s[s.len() - 1];
        }
        extrapolation += start[start.len() - 1];
        extrapolation
    }

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate("0 3 6 9 12 15"), 18);
        assert_eq!(extrapolate("1 3 6 10 15 21"), 28);
        assert_eq!(extrapolate("10 13 16 21 30 45"), 68);
    }
}
