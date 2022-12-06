use std::collections::VecDeque;

use regex::Regex;

use crate::utils::file_read_lines;


#[derive(PartialEq, Eq)]
enum Mode {
    Stacks,
    Instructions
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<VecDeque<char>>
}

impl Stacks {
    fn new(size: usize) -> Stacks {
        Stacks {
            stacks: (0 .. size).map(|_| VecDeque::new()).collect()
        }
    }

    fn add_crate(&mut self, stack: usize, b: char) {
        self.stacks[stack - 1].push_back(b);
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        let b = self.stacks[from - 1].pop_front().unwrap();
        self.stacks[to - 1].push_front(b);
    }

    fn move_crates(&mut self, count: usize, from: usize, to: usize) {
        let crates: Vec<char> = self.stacks[from - 1].drain(0 .. count).collect();
        for x in crates.iter().rev() {
            self.stacks[to - 1].push_front(*x);
        }

    }

    fn tops(&self) -> Vec<char> {
        self.stacks.iter()
            .map(|x| x.front().unwrap().clone())
            .collect()
    }
}

fn process_line_crate(stacks: &mut Stacks, line: String) {
    for x in (1 .. line.len()).step_by(4) {
        let stack_nr = (x / 4) + 1;
        let crate_char = line.chars().nth(x).unwrap();
        if crate_char != ' ' {
            stacks.add_crate(stack_nr, crate_char);
        }
    }
}

fn parse_line_move(line: &String) -> (u32, u32, u32) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)$").unwrap();
    let c = re.captures(&line).unwrap();
    let count = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let from  = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let to    = c.get(3).unwrap().as_str().parse::<u32>().unwrap();
    (count, from, to)
}

fn process_line_move_1(stacks: &mut Stacks, line: String) {
    let (count, from, to) = parse_line_move(&line);

    for _ in 0 .. count {
        stacks.move_crate(from as usize, to as usize);
    }
}

fn process_line_move_2(stacks: &mut Stacks, line: String) {
    let (count, from, to) = parse_line_move(&line);
    stacks.move_crates(count as usize, from as usize, to as usize);
}

fn process_1<T: Iterator<Item = String>>(lines: &mut T) -> Vec<char> {
    let first_line = lines.next().unwrap();
    let stacks_count = (first_line.len() + 1) / 4;
    let mut stacks = Stacks::new(stacks_count);

    process_line_crate(&mut stacks, first_line);

    let mut mode = Mode::Stacks;

    for line in lines {
        if line.is_empty() {
            mode = Mode::Instructions;
        } else if mode == Mode::Stacks {
            process_line_crate(&mut stacks, line);
        } else if mode == Mode::Instructions {
            process_line_move_1(&mut stacks, line)
        }
    }

    stacks.tops()
}

fn process_2<T: Iterator<Item = String>>(lines: &mut T) -> Vec<char> {
    let first_line = lines.next().unwrap();
    let stacks_count = (first_line.len() + 1) / 4;
    let mut stacks = Stacks::new(stacks_count);

    process_line_crate(&mut stacks, first_line);

    let mut mode = Mode::Stacks;

    for line in lines {
        if line.is_empty() {
            mode = Mode::Instructions;
        } else if mode == Mode::Stacks {
            process_line_crate(&mut stacks, line);
        } else if mode == Mode::Instructions {
            process_line_move_2(&mut stacks, line)
        }
    }

    stacks.tops()
}

pub fn run() {
    let lines: Vec<String> = file_read_lines("src/day05/input.txt").unwrap()
        .map(|r| r.unwrap())
        .collect();
    let tops_1 = process_1(&mut lines.iter().map(|x| x.clone()));
    let tops_2 = process_2(&mut lines.iter().map(|x| x.clone()));

    println!("Day05: Output {} and {} ",
        String::from_iter(tops_1),
        String::from_iter(tops_2));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day05::process_2;

    use super::{process_1, Stacks};

    static INPUT: &str = "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n\
        1   2   3 \n\
        \n\
        move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2";

    #[test]
    fn test_move_crate() {
        let mut stacks = Stacks::new(3);
        stacks.add_crate(1, 'X');
        stacks.add_crate(1, 'A');
        stacks.add_crate(2, 'B');
        stacks.add_crate(3, 'C');
        assert_eq!("XBC", String::from_iter(stacks.tops()));

        stacks.move_crate(1, 2);
        assert_eq!("AXC", String::from_iter(stacks.tops()));
    }

    #[test]
    fn test_move_crates() {
        let mut stacks = Stacks::new(3);
        stacks.add_crate(1, 'X');
        stacks.add_crate(1, 'Y');
        stacks.add_crate(1, 'A');
        stacks.add_crate(2, 'B');
        stacks.add_crate(3, 'C');

        stacks.move_crates(2, 1, 2);
        assert_eq!("AXC", String::from_iter(stacks.tops()));
    }

    #[test]
    fn top_crates() {
        let lines: Vec<String> = INPUT.lines()
            .map(|x| String::from_str(x).unwrap())
            .collect();
        let tops = process_1(&mut lines.into_iter());
        assert_eq!("CMZ", String::from_iter(tops));
    }

    #[test]
    fn top_crates_2() {
        let lines: Vec<String> = INPUT.lines()
            .map(|x| String::from_str(x).unwrap())
            .collect();
        let tops = process_2(&mut lines.into_iter());
        assert_eq!("MCD", String::from_iter(tops));
    }
}
