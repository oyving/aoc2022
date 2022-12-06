use std::collections::HashSet;

use crate::utils::file_read_lines;


fn is_marker(n: usize, window: &[char]) -> bool {
    let s: HashSet<&char> = HashSet::from_iter(window.iter());
    s.len() == n
}

fn process(n: usize, line: &str) -> usize {
    let mut count = n;

    for window in line.chars().collect::<Vec<char>>().windows(n) {
        if is_marker(n, window) {
            break;
        }
        count += 1
    }

    count
}

pub fn run() {
    let input = file_read_lines("src/day06/input.txt").unwrap().next().unwrap().unwrap();
    println!("Day06: Output {} - {}",
        process(4, input.as_str()),
        process(14, input.as_str()));
}


#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, process(4, input));
    }

    #[test]
    fn test_2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, process(4, input));
    }

    #[test]
    fn test_3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, process(4, input));
    }

    #[test]
    fn test_4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, process(4, input));
    }

    #[test]
    fn test_5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, process(4, input));
    }

    #[test]
    fn test_6() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, process(14, input));
    }

}