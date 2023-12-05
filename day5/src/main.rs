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
    pub fn handle_input(input: &str) -> u64 {
        *location_indexes(input).iter().min().unwrap()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            35
        );
    }

    fn location_indexes(input: &str) -> Vec<u64> {
        let (seeds, maps) = input.split_once("\n\n").unwrap();
        let mut indexes = parse_seeds(seeds);
        for m in maps.split("\n\n") {
            let map = Map::new(m);
            for s in indexes.iter_mut() {
                let old_index = s.clone();
                *s = map.convert(old_index);
            }
        }
        indexes
    }

    #[test]
    fn test_location_indexes() {
        assert_eq!(
            location_indexes(
                "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            ),
            vec![82, 43, 86, 35]
        );
    }

    struct Map {
        ranges: Vec<MapRange>,
    }

    impl Map {
        fn new(input: &str) -> Self {
            let (_, input) = input.split_once(":").unwrap();
            Self {
                ranges: input
                    .trim()
                    .lines()
                    .map(|l| MapRange::from_str(l))
                    .collect(),
            }
        }

        fn convert(&self, src: u64) -> u64 {
            for r in &self.ranges {
                if r.in_range(src) {
                    return r.destination + src - r.source;
                }
            }
            src
        }
    }

    #[test]
    fn test_convert() {
        let map = Map::new(
            "seed-to-soil map:
50 98 2
52 50 48",
        );
        assert_eq!(map.convert(79), 81);
        assert_eq!(map.convert(14), 14);
        assert_eq!(map.convert(55), 57);
        assert_eq!(map.convert(13), 13);
    }

    #[test]
    fn test_parse_map() {
        let map = Map::new(
            "seed-to-soil map:
50 98 2
52 50 48",
        );
        assert_eq!(map.ranges.len(), 2);
        assert_eq!(map.ranges[0], MapRange::new(50, 98, 2));
    }

    #[derive(Debug, PartialEq)]
    struct MapRange {
        destination: u64,
        source: u64,
        length: u64,
    }

    impl MapRange {
        fn new(destination: u64, source: u64, length: u64) -> Self {
            Self {
                destination,
                source,
                length,
            }
        }

        fn from_str(input: &str) -> Self {
            let re =
                regex::Regex::new(r"(?<destination>\d+) (?<source>\d+) (?<length>\d+)").unwrap();
            let c = re.captures(input).unwrap();
            Self {
                destination: c["destination"].parse::<u64>().unwrap(),
                source: c["source"].parse::<u64>().unwrap(),
                length: c["length"].parse::<u64>().unwrap(),
            }
        }

        fn in_range(&self, src: u64) -> bool {
            return src >= self.source && src < (self.source + self.length);
        }
    }

    #[test]
    fn test_in_range() {
        let mr = MapRange::from_str("50 98 2");
        assert!(!mr.in_range(97));
        assert!(mr.in_range(98));
        assert!(mr.in_range(99));
        assert!(!mr.in_range(100));
    }

    #[test]
    fn test_parse_map_range() {
        assert_eq!(MapRange::from_str("50 98 2"), MapRange::new(50, 98, 2));
        assert_eq!(MapRange::from_str("0 15 37"), MapRange::new(0, 15, 37));
    }

    fn parse_seeds(input: &str) -> Vec<u64> {
        input
            .split_once(":")
            .unwrap()
            .1
            .split(" ")
            .filter_map(|s| s.parse::<u64>().ok())
            .collect()
    }

    #[test]
    fn test_parse_seeds() {
        assert_eq!(parse_seeds("seeds: 79 14 55 13"), vec![79, 14, 55, 13]);
    }
}
