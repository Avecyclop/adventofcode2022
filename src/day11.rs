fn part1(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in monkey.items.iter() {
                monkeys[i].inspects += 1;
                let operation_value = monkey.operation_value.parse::<_>().unwrap_or(*item);
                let mut current_item = match monkey.operation {
                    '*' => *item * operation_value,
                    '+' => *item + operation_value,
                    _ => panic!()
                };
                current_item /= 3;
                if current_item % monkey.test_divisible == 0 {
                    monkeys[monkey.test_true].items.push(current_item);
                } else {
                    monkeys[monkey.test_false].items.push(current_item);
                }
            }
            monkeys[i].items.clear();
        }
    }

    calculate_monkey_business(&mut monkeys)
}

fn part2(input: &str) -> i64 {
    let mut monkeys = parse_monkeys(input);

    let least_common_multiple: i64 = monkeys.iter()
        .map(|monkey| monkey.test_divisible)
        .product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in monkey.items.iter() {
                monkeys[i].inspects += 1;
                let operation_value = monkey.operation_value.parse::<_>().unwrap_or(*item);
                let mut current_item = match monkey.operation {
                    '*' => *item * operation_value,
                    '+' => *item + operation_value,
                    _ => panic!()
                };
                current_item %= least_common_multiple;
                if current_item % monkey.test_divisible == 0 {
                    monkeys[monkey.test_true].items.push(current_item);
                } else {
                    monkeys[monkey.test_false].items.push(current_item);
                }
            }
            monkeys[i].items.clear();
        }
    }

    calculate_monkey_business(&mut monkeys)
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: char,
    operation_value: String,
    test_divisible: i64,
    test_true: usize,
    test_false: usize,
    inspects: i64,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.lines().collect::<Vec<_>>()
        .chunks(7)
        .map(|chunk| {
            Monkey {
                items: chunk[1].split_at(18).1.split(", ").map(|item| item.parse::<_>().unwrap()).collect(),
                operation: chunk[2].chars().collect::<Vec<_>>()[23],
                operation_value: chunk[2].split(" ").last().unwrap().to_string(),
                test_divisible: chunk[3].split(" ").last().unwrap().parse::<_>().unwrap(),
                test_true: chunk[4].split(" ").last().unwrap().parse::<usize>().unwrap(),
                test_false: chunk[5].split(" ").last().unwrap().parse::<usize>().unwrap(),
                inspects: 0,
            }
        })
        .collect()
}

fn calculate_monkey_business(monkeys: &mut Vec<Monkey>) -> i64 {
    monkeys.sort_by_key(|x| { x.inspects });
    monkeys.iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspects)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day11.txt");
        assert_eq!(part1(example), 10605);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day11.txt");
        let result = part1(input);
        println!("Day 11 - Part 1: {}", result);
        assert_eq!(result, 69918);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day11.txt");
        assert_eq!(part2(example), 2_713_310_158);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day11.txt");
        let result = part2(input);
        println!("Day 11 - Part 2: {}", result);
        assert_eq!(result, 19_573_408_701);
    }
}
