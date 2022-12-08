use lazy_static::lazy_static;
use regex::Regex;

fn part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let number_line = lines.iter().position(|line| { STACKS.is_match(line) }).unwrap();
    let stack_amount = get_stacks_amount(&lines, number_line);

    let mut stacks = setup_stacks(&lines, number_line, stack_amount);
    lines.iter()
        .filter(|line| { COMMAND.is_match(line) })
        .map(|command| {
            let captures = COMMAND.captures(command).unwrap();
            (captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
             captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
             captures.get(3).unwrap().as_str().parse::<usize>().unwrap())
        })
        .for_each(|(amount, from_stack, to_stack)| {
            for _ in 0..amount {
                let crate_ = stacks[from_stack - 1].pop().unwrap();
                stacks[to_stack - 1].push(crate_);
            }
        });
    get_string_from_top_of_each_stack(stacks)
}

fn part2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let number_line = lines.iter().position(|line| { STACKS.is_match(line) }).unwrap();
    let stack_amount = get_stacks_amount(&lines, number_line);

    let mut stacks = setup_stacks(&lines, number_line, stack_amount);
    lines.iter()
        .filter(|line| { COMMAND.is_match(line) })
        .map(|command| {
            let captures = COMMAND.captures(command).unwrap();
            (captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
             captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
             captures.get(3).unwrap().as_str().parse::<usize>().unwrap())
        })
        .for_each(|(amount, from_stack, to_stack)| {
            let mut crates: Vec<char> = Vec::new();
            for _ in 0..amount {
                let create_ = stacks[from_stack - 1].pop().unwrap();
                crates.push(create_)
            }
            for _ in 0..amount {
                let crate_ = crates.pop().unwrap();
                stacks[to_stack - 1].push(crate_);
            }
        });
    get_string_from_top_of_each_stack(stacks)
}

lazy_static! {
    static ref STACKS: Regex = Regex::new(r"^(\s*\d+\s*)+$").unwrap();
    static ref COMMAND: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

fn get_string_from_top_of_each_stack(stacks: Vec<Vec<char>>) -> String {
    String::from_iter(stacks.iter()
        .map(|stack| stack.last().unwrap()))
}

fn setup_stacks(lines: &Vec<&str>, number_line: usize, stack_amount: usize) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_amount {
        stacks.push(Vec::new());
    }
    for i in (0..number_line).rev() {
        let chars = lines.get(i).unwrap().chars().collect::<Vec<char>>();
        for j in 0..stack_amount {
            let pos = j * 4 + 1;
            if pos < chars.len() {
                let crate_ = chars[pos];
                if crate_ != ' ' {
                    stacks[j].push(crate_);
                }
            }
        }
    }
    stacks
}

fn get_stacks_amount(lines: &Vec<&str>, number_line: usize) -> usize {
    lines.get(number_line)
        .map(|numbering| {
            numbering.trim()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap()
        })
        .unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day5.txt");
        assert_eq!(part1(example), "CMZ");
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day5.txt");
        let result = part1(input);
        println!("Day 5 - Part 1: {}", result);
        assert_eq!(result, "RTGWZTHLD");
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day5.txt");
        assert_eq!(part2(example), "MCD");
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day5.txt");
        let result = part2(input);
        println!("Day 5 - Part 2: {}", result);
        assert_eq!(result, "STHGRZZFR");
    }
}
