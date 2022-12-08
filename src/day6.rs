fn part1(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for i in 3..chars.len() {
        let a = chars[i - 3];
        let b = chars[i - 2];
        let c = chars[i - 1];
        let d = chars[i];
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return i + 1;
        }
    }
    panic!()
}

fn part2(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    let mut sub_chars: Vec<char> = vec![' '; 14];
    for i in 14..chars.len() {
        sub_chars.clone_from_slice(&chars[i-14..i]);
        sub_chars.sort();
        let any_duplicates = sub_chars.windows(2).any(|c| c[0] == c[1]);
        if !any_duplicates {
            return i;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day6.txt");
        let result = part1(input);
        println!("Day 6 - Part 1: {}", result);
        assert_eq!(result, 1282);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day6.txt");
        let result = part2(input);
        println!("Day 6 - Part 2: {}", result);
        assert_eq!(result, 3513);
    }
}
