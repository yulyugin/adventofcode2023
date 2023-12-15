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

    fn hash(chars: &str) -> u32 {
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
