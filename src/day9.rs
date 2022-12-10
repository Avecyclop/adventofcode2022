use std::collections::HashSet;

fn part1(input: &str) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    input.lines()
        .map(|line| parse_motion(line))
        .for_each(|(direction, paces)| {
            for _ in 0..paces {
                match direction {
                    "U" => {
                        if (head.1 - 1).abs_diff(tail.1) > 1 {
                            tail = head;
                        }
                        head.1 -= 1;
                    }
                    "D" => {
                        if (head.1 + 1).abs_diff(tail.1) > 1 {
                            tail = head;
                        }
                        head.1 += 1;
                    }
                    "L" => {
                        if (head.0 - 1).abs_diff(tail.0) > 1 {
                            tail = head;
                        }
                        head.0 -= 1;
                    }
                    "R" => {
                        if (head.0 + 1).abs_diff(tail.0) > 1 {
                            tail = head;
                        }
                        head.0 += 1;
                    }
                    _ => panic!()
                }
                visited.insert(tail);
            }
        });
    visited.len()
}

fn part2(input: &str) -> usize {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    input.lines()
        .map(|line| parse_motion(line))
        .for_each(|(direction, paces)| for _ in 0..paces {
            match direction {
                "U" => rope[0].1 -= 1,
                "D" => rope[0].1 += 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => panic!()
            }
            for i in 1..rope.len() {
                let diff = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                match diff {
                    //top
                    (0, -2) => rope[i].1 -= 1,
                    //top-right
                    (1, -2) | (2, -1) | (2, -2) => {
                        rope[i].0 += 1;
                        rope[i].1 -= 1;
                    }
                    //right
                    (2, 0) => rope[i].0 += 1,
                    //bottom-right
                    (2, 1) | (1, 2) | (2, 2) => {
                        rope[i].0 += 1;
                        rope[i].1 += 1;
                    }
                    //bottom
                    (0, 2) => rope[i].1 += 1,
                    //bottom-left
                    (-1, 2) | (-2, 1) | (-2, 2) => {
                        rope[i].0 -= 1;
                        rope[i].1 += 1;
                    }
                    //left,
                    (-2, 0) => rope[i].0 -= 1,
                    //top-left
                    (-2, -1) | (-1, -2) | (-2, -2) => {
                        rope[i].0 -= 1;
                        rope[i].1 -= 1;
                    }
                    (_, _) => {}
                }
            }
            visited.insert(rope[9]);
        });
    visited.len()
}

fn parse_motion(line: &str) -> (&str, i32) {
    let motion: Vec<&str> = line.split(" ").collect();
    (motion[0], motion[1].parse::<i32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day9.txt");
        assert_eq!(part1(example), 13);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day9.txt");
        let result = part1(input);
        println!("Day 9 - Part 1: {}", result);
        assert_eq!(result, 6181);
    }

    #[test]
    fn part2_examples() {
        let example1 = include_str!("../example-input/day9.txt");
        assert_eq!(part2(example1), 1);
        let example2 = include_str!("../example-input/day9_2.txt");
        assert_eq!(part2(example2), 36);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day9.txt");
        let result = part2(input);
        println!("Day 9 - Part 2: {}", result);
        assert_eq!(result, 2386);
    }
}
