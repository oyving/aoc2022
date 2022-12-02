use std::path::Path;
use std::option::Option;


use crate::utils::file_read_lines;

fn find_elves_most_food<T: Iterator<Item = Option<u32>>>(calories: T) -> u32
{
    let mut max_food: u32 = 0;
    let mut cur_food: u32 = 0;

    for calorie in calories {
        match calorie {
            Some(number) => {
                cur_food += number;
            },
            None => {
                if cur_food > max_food {
                    max_food = cur_food;
                }
                cur_food = 0;
            }
        };
    }

    max_food
}

fn find_elves_top3_food<T: Iterator<Item = Option<u32>>>(calories: T) -> u32
{
    let mut max_food: [u32; 4] = [0; 4];
    let mut cur_food: u32 = 0;

    for calorie in calories {
        match calorie {
            Some(number) => {
                cur_food += number;
            },
            None => {
                max_food[3] = cur_food;
                max_food.sort();
                max_food.reverse();
                cur_food = 0;
            }
        }
    }
    max_food[3] = cur_food;
    max_food.sort();
    max_food.reverse();
    max_food[0 .. 3].iter().sum()
}

pub fn run() {
    let lines: Vec<Option<u32>> = file_read_lines(Path::new("./src/day01/input.txt")).unwrap()
        .map(|line_result| {
            let line = line_result.unwrap();
            if line.is_empty() {
                None
            } else {
                Some(line.parse::<u32>().unwrap())
            }
        })
        .collect();
    let most_food = find_elves_most_food(lines.iter().copied());
    let top3_food: u32 = find_elves_top3_food(lines.iter().copied());
    println!("Day01: Most food {}", most_food);
    println!("Day01: Top3 food {}", top3_food);
}


#[cfg(test)]
mod tests {
    use crate::day01::{find_elves_most_food, find_elves_top3_food};

    static INPUT: &str = "1000\n\
        2000\n\
        3000\n\
        \n\
        4000\n\
        \n\
        5000\n\
        6000\n\
        \n\
        7000\n\
        8000\n\
        9000\n\
        \n\
        10000";

    #[test]
    fn test_most_food() {
        let lines = INPUT.lines()
            .map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(line.parse::<u32>().unwrap())
                }
            });
        let most_food = find_elves_most_food(lines);
        assert_eq!(most_food, 24000);
    }

    #[test]
    fn test_top3_food() {
        let lines = INPUT.lines()
            .map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(line.parse::<u32>().unwrap())
                }
            });
        let top3_food = find_elves_top3_food(lines);
        assert_eq!(top3_food, 45000);
    }
}

