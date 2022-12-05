fn part1(input: &str) -> i32 {
    input.lines()
        .map(|line| {
            match line {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => panic!()
            }
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input.lines()
        .map(|line| {
            match line {
                "A X" => 3 + 0,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 2 + 0,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => panic!()
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day2.txt");
        assert_eq!(part1(example), 15);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day2.txt");
        let result = part1(input);
        println!("Day 2 - Part 1: {}", result);
        assert_eq!(result, 14163);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day2.txt");
        assert_eq!(part2(example), 12);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day2.txt");
        let result = part2(input);
        println!("Day 2 - Part 2: {}", result);
        assert_eq!(result, 12091);
    }
}
