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
    use std::collections::HashMap;

    struct Node {
        left: String,
        right: String,
    }

    impl Node {
        fn from_str(input: &str) -> Self {
            let re = regex::Regex::new(r"\((?<left>[A-Z]+), (?<right>[A-Z]+)\)").unwrap();
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

    fn parse_nodes(input: &str) -> HashMap<String, Node> {
        let mut results = HashMap::new();
        for l in input.lines() {
            let (key, value) = l.split_once("=").unwrap();
            results.insert(key.trim().to_string(), Node::from_str(value));
        }
        results
    }

    pub fn handle_input(input: &str) -> u32 {
        let (commands, nodes) = input.split_once("\n\n").unwrap();
        let commands = parse_commands(commands);
        let nodes = parse_nodes(nodes);

        let mut result = 0;
        let mut current = &nodes[&"AAA".to_string()];
        loop {
            for c in &commands {
                let next = current.next(c);
                result += 1;
                if next == "ZZZ" {
                    return result;
                }
                current = &nodes[next];
            }
        }
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
    enum Command {
        Left,
        Right,
    }

    fn parse_commands(input: &str) -> Vec<Command> {
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
