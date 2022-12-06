fn part1(input: &str) -> i32 {
    input.lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(compartment1, compartment2)| {
            for item in compartment1.chars() {
                if compartment2.contains(item) {
                    return item;
                }
            }
            panic!()
        })
        .map(|item| item_to_priority(item))
        .sum()
}

fn part2(input: &str) -> i32 {
    input.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| (
            chunk.get(0).unwrap(),
            chunk.get(1).unwrap(),
            chunk.get(2).unwrap())
        )
        .map(|(rucksack1, rucksack2, rucksack3)| {
            for item in rucksack1.chars() {
                if rucksack2.contains(item) && rucksack3.contains(item) {
                    return item;
                }
            }
            panic!()
        })
        .map(|item| item_to_priority(item))
        .sum()
}

fn item_to_priority(item: char) -> i32 {
    if item.is_uppercase() {
        item as i32 - 38
    } else {
        item as i32 - 96
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day3.txt");
        assert_eq!(part1(example), 157);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day3.txt");
        let result = part1(input);
        println!("Day 3 - Part 1: {}", result);
        assert_eq!(result, 8153);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day3.txt");
        assert_eq!(part2(example), 70);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day3.txt");
        let result = part2(input);
        println!("Day 3 - Part 2: {}", result);
        assert_eq!(result, 2342);
    }
}
