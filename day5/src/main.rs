use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("Task1 answer: {}", task1::handle_input(&input));
    println!("Task1 answer: {}", task2::handle_input(&input));

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

    pub struct Map {
        pub ranges: Vec<MapRange>,
    }

    impl Map {
        pub fn new(input: &str) -> Self {
            let (_, input) = input.split_once(":").unwrap();
            Self {
                ranges: input
                    .trim()
                    .lines()
                    .map(|l| MapRange::from_str(l))
                    .collect(),
            }
        }

        pub fn convert(&self, src: u64) -> u64 {
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
    pub struct MapRange {
        pub destination: u64,
        pub source: u64,
        pub length: u64,
    }

    impl MapRange {
        #[cfg(test)]
        pub fn new(destination: u64, source: u64, length: u64) -> Self {
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

mod task2 {
    use crate::task1::{Map, MapRange};

    pub fn handle_input(input: &str) -> u64 {
        location_indexes(input)
            .iter()
            .map(|l| l.start)
            .min()
            .unwrap()
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
            46
        );
    }

    fn location_indexes(input: &str) -> Vec<PointRange> {
        let (seeds, maps) = input.split_once("\n\n").unwrap();
        let mut points = Points::from_str(seeds);
        for m in maps.split("\n\n") {
            let map = Map::new(m);
            points = map.convert_points(points);
        }
        points.ranges
    }

    impl MapRange {
        fn convert_point_range(&self, points: PointRange) -> (Points, Points) {
            let empty = Points::empty();

            let points_start = points.start;
            let points_end = points.start + points.length;

            let range_start = self.source;
            let range_end = self.source + self.length;

            if points_start < range_start {
                if points_end < range_start {
                    return (Points::new(vec![points]), empty);
                } else if points_end <= range_end {
                    return (
                        Points::new(vec![PointRange::new(
                            points_start,
                            range_start - points_start,
                        )]),
                        Points::new(vec![PointRange::new(
                            self.destination,
                            points_end - range_start,
                        )]),
                    );
                } else {
                    return (
                        Points::new(vec![
                            PointRange::new(points_start, range_start - points_start),
                            PointRange::new(range_end, points_end - range_end),
                        ]),
                        Points::new(vec![PointRange::new(self.destination, self.length)]),
                    );
                }
            } else {
                if points_start > range_end {
                    return (Points::new(vec![points]), empty);
                } else if points_end <= range_end {
                    return (
                        empty,
                        Points::new(vec![PointRange::new(
                            self.destination + points_start - range_start,
                            points.length,
                        )]),
                    );
                } else {
                    return (
                        Points::new(vec![PointRange::new(range_end, points_end - range_end)]),
                        Points::new(vec![PointRange::new(
                            self.destination + points_start - range_start,
                            range_end - points_start,
                        )]),
                    );
                }
            }
        }

        fn convert_points(&self, points: Points) -> (Points, Points) {
            let mut unmodified = Points::empty();
            let mut converted = Points::empty();
            for p in points.ranges {
                let (mut u, mut c) = self.convert_point_range(p);
                unmodified.append(&mut u);
                converted.append(&mut c);
            }
            (unmodified, converted)
        }
    }

    #[test]
    fn test_convert_point_range() {
        assert_eq!(
            MapRange::new(100, 8, 2).convert_point_range(PointRange::new(0, 5)),
            (Points::new(vec![PointRange::new(0, 5)]), Points::empty())
        );
        assert_eq!(
            MapRange::new(100, 8, 2).convert_point_range(PointRange::new(20, 5)),
            (Points::new(vec![PointRange::new(20, 5)]), Points::empty())
        );
        assert_eq!(
            MapRange::new(100, 8, 2).convert_point_range(PointRange::new(0, 9)),
            (
                Points::new(vec![PointRange::new(0, 8)]),
                Points::new(vec![PointRange::new(100, 1)])
            )
        );
        assert_eq!(
            MapRange::new(100, 8, 2).convert_point_range(PointRange::new(0, 12)),
            (
                Points::new(vec![PointRange::new(0, 8), PointRange::new(10, 2)]),
                Points::new(vec![PointRange::new(100, 2)])
            )
        );
        assert_eq!(
            MapRange::new(100, 8, 2).convert_point_range(PointRange::new(9, 12)),
            (
                Points::new(vec![PointRange::new(10, 11)]),
                Points::new(vec![PointRange::new(101, 1)])
            )
        );
        assert_eq!(
            MapRange::new(100, 8, 2).convert_point_range(PointRange::new(9, 1)),
            (Points::empty(), Points::new(vec![PointRange::new(101, 1)]))
        );
        assert_eq!(
            MapRange::new(100, 8, 2).convert_point_range(PointRange::new(0, 8)),
            (Points::new(vec![PointRange::new(0, 8)]), Points::empty())
        );
    }

    impl Map {
        fn convert_points(&self, points: Points) -> Points {
            let mut converted = Points::empty();
            let mut unmodified = points;

            let mut m_iter = self.ranges.iter();
            while let Some(m) = m_iter.next() {
                let (u, mut c) = m.convert_points(unmodified);
                unmodified = u;
                converted.append(&mut c);
            }
            converted.append(&mut unmodified);
            converted
        }
    }

    #[derive(Debug, PartialEq)]
    struct Points {
        ranges: Vec<PointRange>,
    }

    impl Points {
        fn new(ranges: Vec<PointRange>) -> Self {
            Self {
                ranges: ranges
                    .into_iter()
                    .filter(|r| r.length != 0)
                    .collect::<Vec<PointRange>>(),
            }
        }

        fn empty() -> Self {
            Self { ranges: vec![] }
        }

        fn append(&mut self, points: &mut Points) {
            self.ranges.append(&mut points.ranges)
        }

        fn from_str(input: &str) -> Self {
            let (_, seed_specs) = input.split_once(":").unwrap();
            let re = regex::Regex::new(r"(?<start>\d+) (?<length>\d+)").unwrap();
            let mut ranges = vec![];
            for c in re.captures_iter(seed_specs) {
                ranges.push(PointRange::new(
                    c["start"].parse::<u64>().unwrap(),
                    c["length"].parse::<u64>().unwrap(),
                ));
            }
            Self { ranges }
        }
    }

    #[test]
    fn test_points_append() {
        let mut points = Points::empty();
        points.append(&mut Points::empty());
        assert_eq!(points, Points::empty());

        points.append(&mut Points::new(vec![PointRange::new(0, 1)]));
        assert_eq!(points, Points::new(vec![PointRange::new(0, 1)]));

        points.append(&mut Points::new(vec![
            PointRange::new(1, 1),
            PointRange::new(2, 1),
        ]));
        assert_eq!(
            points,
            Points::new(vec![
                PointRange::new(0, 1),
                PointRange::new(1, 1),
                PointRange::new(2, 1)
            ])
        );
    }

    #[derive(Debug, PartialEq)]
    struct PointRange {
        start: u64,
        length: u64,
    }

    impl PointRange {
        fn new(start: u64, length: u64) -> Self {
            Self { start, length }
        }
    }

    #[test]
    fn test_parse_points() {
        assert_eq!(
            Points::from_str("seeds: 79 14 55 13").ranges,
            vec![PointRange::new(79, 14), PointRange::new(55, 13)]
        );
    }
}
