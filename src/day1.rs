use std::iter::Map;
use std::str::Split;

fn part1(input: &str) -> i32 {
    parse_calories(input)
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let mut elves = parse_calories(input)
        .collect::<Vec<i32>>();
    elves.sort();
    elves.reverse();
    elves.iter()
        .take(3)
        .sum()
}

fn parse_calories(input: &str) -> Map<Split<&str>, fn(&str) -> i32> {
    input.trim()
        .split("\r\n\r\n")
        .map(|chunks| chunks
            .split("\r\n")
            .map(|cal| cal
                .trim()
                .parse::<i32>()
                .unwrap())
            .sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day1.txt");
        assert_eq!(part1(example), 24000);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day1.txt");
        let result = part1(input);
        println!("Day 1 - Part 1: {}", result);
        assert_eq!(result, 72718);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day1.txt");
        assert_eq!(part2(example), 45000);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day1.txt");
        let result = part2(input);
        println!("Day 1 - Part 2: {}", result);
        assert_eq!(result, 213089);
    }
}
