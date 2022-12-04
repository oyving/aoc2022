use std::collections::HashSet;

use crate::utils::file_read_lines;

struct Rucksack {
    a: Vec<char>,
    b: Vec<char>,
}

impl Rucksack {
    fn from(line: &str) -> Rucksack {
        let p = line.len() / 2;
        let s = line.split_at(p);
        Rucksack {
            a: s.0.chars().collect(),
            b: s.1.chars().collect(),
        }
    }

    fn score(&self) -> u32 {
        let mut dups: Vec<char> = Vec::new();
        for x in self.a.iter() {
            if self.b.contains(&x) && !dups.contains(&x) {
                dups.push(*x);
            }
        }
        dups.iter().map(score_char).sum()
    }

    fn unique(&self) -> HashSet<char> {
        let mut set: HashSet<char> = HashSet::new();
        self.a.iter().for_each(|x| {
            set.insert(*x);
        });
        self.b.iter().for_each(|x| {
            set.insert(*x);
        });
        set
    }
}

fn badge(a: &Rucksack, b: &Rucksack, c: &Rucksack) -> char {
    let mut au = a.unique();
    let bu = b.unique();
    let cu = c.unique();

    au.retain(|x| bu.contains(x));
    au.retain(|x| cu.contains(x));

    au.iter().next().unwrap().clone()
}

fn score_char(char: &char) -> u32 {
    if char.is_ascii_lowercase() {
        (*char as u32) - 96
    } else {
        (*char as u32) - 64 + 26
    }
}

pub fn run() {
    let rucksacks: Vec<Rucksack> = file_read_lines("src/day03/input.txt")
        .unwrap()
        .map(|line| line.unwrap())
        .map(|line| Rucksack::from(line.as_str()))
        .collect();

    let score_1: u32 = rucksacks.iter().map(|x| x.score()).sum();

    let score_2: u32 = rucksacks
        .chunks(3)
        .map(|c| badge(&c[0], &c[1], &c[2]))
        .map(|c| score_char(&c))
        .sum();

    println!("Day03: Score 1 {}", score_1);
    println!("Day03: Score 2 {}", score_2);
}

#[cfg(test)]
mod tests {
    use super::{score_char, badge};
    use super::Rucksack;

    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        let mut score = 0;
        for line in INPUT.lines() {
            let rucksack = Rucksack::from(line);
            score += rucksack.score();
        }
        assert_eq!(157, score);
    }

    #[test]
    fn test_part_2() {
        let rucksacks: Vec<Rucksack> = INPUT.lines().map(Rucksack::from).collect();
        let score: u32 = rucksacks
            .chunks(3)
            .map(|c| badge(&c[0], &c[1], &c[2]))
            .map(|c| score_char(&c))
            .sum();
        assert_eq!(70, score);
    }
}
