use crate::utils::file_read_lines;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from(line: &str) -> Range {
        let split: Vec<&str> = line.split("-").collect();
        Range {
            start: split[0].parse().unwrap(),
            end: split[1].parse().unwrap(),
        }
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.start
    }
}

struct Assignment {
    first: Range,
    second: Range,
}

impl Assignment {
    fn from(line: &str) -> Assignment {
        let split: Vec<&str> = line.split(",").collect();
        Assignment {
            first: Range::from(split[0]),
            second: Range::from(split[1]),
        }
    }

    fn contains(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    fn overlaps(&self) -> bool {
        self.first.overlaps(&self.second) || self.second.overlaps(&self.first)
    }
}

pub fn run() {
    let assignments: Vec<Assignment> = file_read_lines("src/day04/input.txt").unwrap()
        .map(Result::unwrap)
        .map(|x| Assignment::from(x.as_str()))
        .collect();

    let contains = assignments.iter()
        .filter(|x| Assignment::contains(&x))
        .count();
    
    let overlaps = assignments.iter()
        .filter(|x| Assignment::overlaps(&x))
        .count();
    
    println!("Day04: Count {} and {}", contains, overlaps);
}

#[cfg(test)]
mod tests {
    use super::Assignment;

    static INPUT: &str = "2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8";
    
    #[test]
    fn fully_contains() {
        let count = INPUT.lines()
            .map(Assignment::from)
            .filter(Assignment::contains)
            .count();
        assert_eq!(2, count);
    }

    #[test]
    fn overlaps() {
        let count = INPUT.lines()
            .map(Assignment::from)
            .filter(Assignment::overlaps)
            .count();
        assert_eq!(4, count);
    }
}
