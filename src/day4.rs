use std::iter::Map;
use std::str::Lines;

fn part1(input: &str) -> usize {
    parse_assignments(input)
        .filter(|(a1, a2, b1, b2)| {
            b1 <= a1 && a2 <= b2 ||
                a1 <= b1 && b2 <= a2
        })
        .count()
}

fn part2(input: &str) -> usize {
    parse_assignments(input)
        .filter(|(a1, a2, b1, b2)| {
            a1 <= b1 && b1 <= a2 ||
                a1 <= b2 && b2 <= a2 ||
                b1 <= a1 && a1 <= b2 ||
                b1 <= a2 && a2 <= b2
        })
        .count()
}

fn parse_assignments(input: &str) -> Map<Lines, fn(&str) -> (i32, i32, i32, i32)> {
    input.lines()
        .map(|line| {
            let sections = line.split(",").collect::<Vec<&str>>();
            let section1 = sections.first().unwrap().split("-").collect::<Vec<&str>>();
            let section2 = sections.last().unwrap().split("-").collect::<Vec<&str>>();
            (section1.first().unwrap().parse::<i32>().unwrap(),
             section1.last().unwrap().parse::<i32>().unwrap(),
             section2.first().unwrap().parse::<i32>().unwrap(),
             section2.last().unwrap().parse::<i32>().unwrap())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day4.txt");
        assert_eq!(part1(example), 2);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day4.txt");
        let result = part1(input);
        println!("Day 4 - Part 1: {}", result);
        assert_eq!(result, 550);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day4.txt");
        assert_eq!(part2(example), 4);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day4.txt");
        let result = part2(input);
        println!("Day 4 - Part 2: {}", result);
        assert_eq!(result, 931);
    }
}
