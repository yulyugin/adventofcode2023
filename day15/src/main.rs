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
    pub fn handle_input(input: &str) -> u32 {
        input.trim().split(",").map(|l| hash(l)).sum()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }

    pub fn hash(chars: &str) -> u32 {
        let mut value = 0;
        chars
            .as_bytes()
            .iter()
            .for_each(|c| value = (value + *c as u32) * 17 % 256);
        value
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("H"), 200);
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
    }
}

mod task2 {
    use crate::task1::hash;

    #[derive(Clone, Debug, PartialEq)]
    struct Lens {
        label: String,
        focal_length: usize,
    }

    impl Lens {
        fn new(label: &str, focal_length: usize) -> Self {
            Self {
                label: label.to_string(),
                focal_length,
            }
        }
    }

    #[derive(Clone)]
    struct Box {
        lenses: Vec<Lens>,
    }

    impl Box {
        fn new() -> Self {
            Self { lenses: vec![] }
        }

        fn remove_lens(&mut self, label: &str) {
            for i in 0..self.lenses.len() {
                let l = &self.lenses[i];
                if l.label == label {
                    self.lenses.remove(i);
                    return;
                }
            }
        }

        fn add_lens(&mut self, label: &str, focal_length: usize) {
            for i in 0..self.lenses.len() {
                let l = &mut self.lenses[i];
                if l.label == label {
                    l.focal_length = focal_length;
                    return;
                }
            }
            self.lenses.push(Lens::new(label, focal_length));
        }
    }

    struct Boxes {
        boxes: Vec<Box>,
    }

    impl Boxes {
        fn new() -> Self {
            Self {
                boxes: vec![Box::new(); 256],
            }
        }

        fn handle_command(&mut self, command: &str) {
            if command.ends_with("-") {
                let label = &command[0..command.len() - 1];
                let b = &mut self.boxes[hash(label) as usize];
                b.remove_lens(label);
            } else {
                let (label, focal_length) = command.split_once("=").unwrap();
                let b = &mut self.boxes[hash(label) as usize];
                b.add_lens(label, focal_length.parse::<usize>().unwrap());
            }
        }
    }

    #[test]
    fn test_handle_command() {
        let mut b = Boxes::new();

        b.handle_command("rn=1");
        assert_eq!(b.boxes[0].lenses, vec![Lens::new("rn", 1)]);

        b.handle_command("cm-");
        assert_eq!(b.boxes[0].lenses, vec![Lens::new("rn", 1)]);

        b.handle_command("qp=3");
        assert_eq!(b.boxes[1].lenses, vec![Lens::new("qp", 3)]);

        b.handle_command("cm=2");
        assert_eq!(
            b.boxes[0].lenses,
            vec![Lens::new("rn", 1), Lens::new("cm", 2)]
        );

        b.handle_command("qp-");
        assert_eq!(
            b.boxes[0].lenses,
            vec![Lens::new("rn", 1), Lens::new("cm", 2)]
        );
        assert_eq!(b.boxes[1].lenses, vec![]);

        b.handle_command("pc=4");
        b.handle_command("ot=9");
        b.handle_command("ab=5");
        assert_eq!(
            b.boxes[3].lenses,
            vec![Lens::new("pc", 4), Lens::new("ot", 9), Lens::new("ab", 5)]
        );

        b.handle_command("pc-");
        b.handle_command("pc=6");
        assert_eq!(
            b.boxes[3].lenses,
            vec![Lens::new("ot", 9), Lens::new("ab", 5), Lens::new("pc", 6)]
        );

        b.handle_command("ot=7");
        assert_eq!(
            b.boxes[3].lenses,
            vec![Lens::new("ot", 7), Lens::new("ab", 5), Lens::new("pc", 6)]
        );
    }

    pub fn handle_input(input: &str) -> usize {
        let mut b = Boxes::new();
        input.trim().split(",").for_each(|c| b.handle_command(c));
        b.boxes
            .iter()
            .enumerate()
            .map(|(box_id, b)| {
                b.lenses
                    .iter()
                    .enumerate()
                    .map(|(slot_id, l)| l.focal_length * (slot_id + 1))
                    .sum::<usize>()
                    * (box_id + 1)
            })
            .sum()
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(
            handle_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
