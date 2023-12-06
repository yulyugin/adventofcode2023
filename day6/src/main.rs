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
    use std::iter::zip;

    pub fn handle_input(input: &str) -> u64 {
        let mut result = 1;
        for ways_to_win in parse_races(input).iter().map(|r| r.number_of_ways_to_win()) {
            result *= ways_to_win
        }
        result
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            288
        );
    }

    #[derive(Debug, PartialEq)]
    pub struct Race {
        time: u64,
        record: u64,
    }

    impl Race {
        pub fn new(time: u64, record: u64) -> Self {
            Self { time, record }
        }

        fn record_breaking_holds(&self) -> Vec<u64> {
            let mut results = vec![];
            for hold_time in 0..self.time + 1 {
                let speed = hold_time;
                let move_time = self.time - hold_time;
                let distance = speed * move_time;
                if distance > self.record {
                    results.push(hold_time);
                }
            }
            results
        }

        pub fn number_of_ways_to_win(&self) -> u64 {
            self.record_breaking_holds().len() as u64
        }
    }

    #[test]
    fn test_number_of_ways_to_win() {
        assert_eq!(Race::new(7, 9).number_of_ways_to_win(), 4);
        assert_eq!(Race::new(15, 40).number_of_ways_to_win(), 8);
        assert_eq!(Race::new(30, 200).number_of_ways_to_win(), 9);
    }

    #[test]
    fn test_record_breaking_holds() {
        assert_eq!(Race::new(7, 9).record_breaking_holds(), vec![2, 3, 4, 5]);
    }

    fn parse_races(input: &str) -> Vec<Race> {
        let (times, records) = input.split_once("\n").unwrap();
        let times: Vec<u64> = times
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(" ")
            .filter_map(|i| i.parse::<u64>().ok())
            .collect();
        let records: Vec<u64> = records
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split(" ")
            .filter_map(|i| i.parse::<u64>().ok())
            .collect();
        zip(times, records)
            .map(|(time, record)| Race { time, record })
            .collect()
    }

    #[test]
    fn test_parse_races() {
        assert_eq!(
            parse_races(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        );
    }
}

mod task2 {
    use crate::task1::Race;

    pub fn handle_input(input: &str) -> u64 {
        parse_race(input).number_of_ways_to_win()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            71503
        );
    }

    fn parse_race(input: &str) -> Race {
        let (time, record) = input.trim().split_once("\n").unwrap();
        let time: u64 = time
            .split_once(":")
            .unwrap()
            .1
            .replace(" ", "")
            .parse::<u64>()
            .unwrap();
        let record: u64 = record
            .split_once(":")
            .unwrap()
            .1
            .replace(" ", "")
            .parse::<u64>()
            .unwrap();
        Race::new(time, record)
    }

    #[test]
    fn test_parse_races() {
        assert_eq!(
            parse_race(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            Race::new(71530, 940200)
        );
    }
}
