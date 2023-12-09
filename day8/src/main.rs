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
    use std::collections::HashMap;

    pub struct Node {
        left: String,
        right: String,
    }

    impl Node {
        fn from_str(input: &str) -> Self {
            let re = regex::Regex::new(r"\((?<left>[A-Z0-9]+), (?<right>[A-Z0-9]+)\)").unwrap();
            let m = re.captures(input).unwrap();
            Self {
                left: m["left"].trim().to_string(),
                right: m["right"].trim().to_string(),
            }
        }

        fn next(&self, command: &Command) -> &String {
            match command {
                Command::Left => &self.left,
                Command::Right => &self.right,
            }
        }
    }

    #[test]
    fn test_node_from_string() {
        let node = Node::from_str("(BBB, CCC)");
        assert_eq!(node.left, "BBB");
        assert_eq!(node.right, "CCC");
    }

    pub fn parse_nodes(input: &str) -> HashMap<String, Node> {
        let mut results = HashMap::new();
        for l in input.lines() {
            let (key, value) = l.split_once("=").unwrap();
            results.insert(key.trim().to_string(), Node::from_str(value));
        }
        results
    }

    pub fn get_number_of_steps(
        start: &String,
        commands: &Vec<Command>,
        nodes: &HashMap<String, Node>,
        end_condition: fn(&String) -> bool,
    ) -> usize {
        let mut current = &nodes[start];
        for step in 0.. {
            let c = &commands[step % commands.len()];
            let next = current.next(c);
            if end_condition(next) {
                return step + 1;
            }
            current = &nodes[next];
        }
        unreachable!();
    }

    pub fn handle_input(input: &str) -> usize {
        let (commands, nodes) = input.split_once("\n\n").unwrap();
        let commands = parse_commands(commands);
        let nodes = parse_nodes(nodes);
        get_number_of_steps(&"AAA".to_string(), &commands, &nodes, |s| s == "ZZZ")
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );
        assert_eq!(
            handle_input(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[derive(Debug, PartialEq)]
    pub enum Command {
        Left,
        Right,
    }

    pub fn parse_commands(input: &str) -> Vec<Command> {
        let mut commands = vec![];
        for c in input.chars() {
            match c {
                'L' => commands.push(Command::Left),
                'R' => commands.push(Command::Right),
                _ => panic!("unreachable"),
            }
        }
        commands
    }

    #[test]
    fn test_parse_commands() {
        assert_eq!(parse_commands("RL"), vec![Command::Right, Command::Left]);
        assert_eq!(
            parse_commands("LLR"),
            vec![Command::Left, Command::Left, Command::Right]
        );
    }
}

mod task2 {
    use crate::task1::{get_number_of_steps, parse_commands, parse_nodes};

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm<I>(nums: I) -> usize
    where
        I: Iterator<Item = usize>,
    {
        nums.fold(1, |num, ans| num * ans / gcd(num, ans))
    }

    pub fn handle_input(input: &str) -> usize {
        let (commands, nodes) = input.split_once("\n\n").unwrap();
        let commands = parse_commands(commands);
        let nodes = parse_nodes(nodes);

        let starts: Vec<&String> = nodes.keys().filter(|k| k.ends_with('A')).collect();
        lcm(starts
            .iter()
            .map(|s| get_number_of_steps(s, &commands, &nodes, |s| s.ends_with('Z'))))
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}
