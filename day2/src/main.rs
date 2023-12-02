use std::fs::File;
use std::io::prelude::*;

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

mod task1 {
    use std::collections::HashMap;

    pub fn handle_input(input: &str) -> u32 {
        let mut ret = 0;
        for l in input.lines() {
            ret += line_value(l);
        }
        ret
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    fn line_value(line: &str) -> u32 {
        let (index, game) = line.trim().split_once(":").unwrap();
        if is_possible_game(game) {
            game_index(index)
        } else {
            0
        }
    }

    #[test]
    fn test_line_value() {
        assert_eq!(
            line_value("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            line_value("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            2
        );
        assert_eq!(
            line_value(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        "
            ),
            0
        );
        assert_eq!(
            line_value(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        "
            ),
            0
        );
        assert_eq!(
            line_value(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "
            ),
            5
        );
    }

    fn is_possible_game(game: &str) -> bool {
        for reach in game.split(";") {
            if !is_possible_reach(reach) {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_is_possible_game() {
        assert_eq!(
            is_possible_game("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            true
        );
        assert_eq!(
            is_possible_game("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            true
        );
        assert_eq!(
            is_possible_game("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            false
        );
        assert_eq!(
            is_possible_game("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            false
        );
        assert_eq!(
            is_possible_game("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            true
        );
    }

    fn is_possible_reach(reach: &str) -> bool {
        let color_map: HashMap<&str, u32> =
            HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        for s in reach.split(",") {
            let (val, color) = s.trim().split_once(" ").unwrap();
            if color_map[color] < val.parse::<u32>().unwrap() {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_is_possible_reach() {
        assert_eq!(is_possible_reach("3 blue"), true);
        assert_eq!(is_possible_reach("12 red"), true);
        assert_eq!(is_possible_reach("13 red"), false);
        assert_eq!(is_possible_reach("13 green"), true);
        assert_eq!(is_possible_reach("14 green"), false);
        assert_eq!(is_possible_reach("14 blue"), true);
        assert_eq!(is_possible_reach("15 blue"), false);

        assert_eq!(is_possible_reach("3 blue, 4 red"), true);
        assert_eq!(is_possible_reach("1 red, 2 green, 6 blue"), true);
        assert_eq!(is_possible_reach("8 green, 6 blue, 20 red"), false);
    }

    fn game_index(game_spec: &str) -> u32 {
        let index_str = game_spec.split(" ").collect::<Vec<&str>>()[1];
        index_str.parse::<u32>().unwrap()
    }

    #[test]
    fn test_game_index() {
        assert_eq!(game_index("Game 1"), 1);
        assert_eq!(game_index("Game 10"), 10);
        assert_eq!(game_index("Game 29"), 29);
    }
}

mod task2 {
    use std::cmp::max;
    use std::collections::HashMap;

    pub fn handle_input(input: &str) -> u32 {
        let mut ret = 0;
        for l in input.lines() {
            ret += line_value(l);
        }
        ret
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }

    fn line_value(line: &str) -> u32 {
        let (_, game) = line.trim().split_once(":").unwrap();
        let (r, g, b) = game_values(game);
        r * g * b
    }

    #[test]
    fn test_line_value() {
        assert_eq!(
            line_value("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            line_value("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            12
        );
        assert_eq!(
            line_value("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            1560
        );
        assert_eq!(
            line_value("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            630
        );
        assert_eq!(
            line_value("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            36
        );
    }

    fn game_values(game: &str) -> (u32, u32, u32) {
        let mut ret = (0, 0, 0);
        for reach in game.split(";") {
            let rgb = reach_values(reach);
            ret = (max(rgb.0, ret.0), max(rgb.1, ret.1), max(rgb.2, ret.2));
        }
        ret
    }

    #[test]
    fn test_game_values() {
        assert_eq!(
            game_values("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            (4, 2, 6)
        );
        assert_eq!(
            game_values("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            (1, 3, 4)
        );
        assert_eq!(
            game_values("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            (20, 13, 6)
        );
        assert_eq!(
            game_values("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            (14, 3, 15)
        );
        assert_eq!(
            game_values("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            (6, 3, 2)
        );
    }

    fn reach_values(reach: &str) -> (u32, u32, u32) {
        let mut color_values: HashMap<&str, u32> = HashMap::new();
        for s in reach.split(",") {
            let (val, color) = s.trim().split_once(" ").unwrap();
            color_values.insert(color, val.parse::<u32>().unwrap());
        }
        (
            *color_values.get("red").unwrap_or(&0),
            *color_values.get("green").unwrap_or(&0),
            *color_values.get("blue").unwrap_or(&0),
        )
    }

    #[test]
    fn test_reach_values() {
        assert_eq!(reach_values("3 blue, 4 red"), (4, 0, 3));
        assert_eq!(reach_values("1 red, 2 green, 6 blue"), (1, 2, 6));
        assert_eq!(reach_values("2 green"), (0, 2, 0));
        assert_eq!(reach_values("1 blue, 2 green"), (0, 2, 1));
        assert_eq!(reach_values("3 green, 4 blue, 1 red"), (1, 3, 4));
        assert_eq!(reach_values("1 green, 1 blue"), (0, 1, 1));
        assert_eq!(reach_values("8 green, 6 blue, 20 red"), (20, 8, 6));
        assert_eq!(reach_values("5 blue, 4 red, 13 green"), (4, 13, 5));
        assert_eq!(reach_values("5 green, 1 red"), (1, 5, 0));
        assert_eq!(reach_values("1 green, 3 red, 6 blue"), (3, 1, 6));
        assert_eq!(reach_values("3 green, 6 red"), (6, 3, 0));
        assert_eq!(reach_values("3 green, 15 blue, 14 red"), (14, 3, 15));
        assert_eq!(reach_values("6 red, 1 blue, 3 green"), (6, 3, 1));
        assert_eq!(reach_values("2 blue, 1 red, 2 green"), (1, 2, 2));
    }
}
