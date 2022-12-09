use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let dirs = parse_commands_into_dir_sizes(input);

    dirs.values()
        .filter(|size| **size <= 100_000)
        .sum()
}

fn part2(input: &str) -> i32 {
    let dirs = parse_commands_into_dir_sizes(input);
    let unused_space = 70_000_000 - dirs.values().max().unwrap();
    let space_to_free = 30_000_000 - unused_space;

    *dirs.values()
        .filter(|size| **size >= space_to_free)
        .min()
        .unwrap()
}

fn parse_commands_into_dir_sizes(input: &str) -> HashMap<String, i32> {
    let mut dirs: HashMap<String, i32> = HashMap::new();
    let mut current_dirs: Vec<String> = Vec::new();
    let mut current_path: String = String::new();
    input.lines().for_each(|line| {
        let cmd: Vec<&str> = line.split(" ").collect();
        if cmd[0] == "$" {
            if cmd[1] == "cd" {
                if cmd[2] == ".." {
                    let last_slash = current_path.rfind("/").unwrap();
                    current_path = current_path.split_at(last_slash).0.to_string();
                    current_dirs.pop();
                } else {
                    current_path.push_str("/");
                    current_path.push_str(cmd[2]);
                    current_dirs.push(current_path.clone());
                }
            } else if cmd[1] == "ls" {
                //Nothing
            }
        } else if cmd[0] == "dir" {
            //Nothing
        } else {
            current_dirs.iter().for_each(|dir| {
                let file_size = cmd[0].parse::<i32>().unwrap();
                *dirs.entry(dir.to_string()).or_insert(0) += file_size;
            })
        }
    });
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day7.txt");
        assert_eq!(part1(example), 95437);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day7.txt");
        let result = part1(input);
        println!("Day 7 - Part 1: {}", result);
        assert_ne!(result, 827083);
        assert_eq!(result, 1501149);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day7.txt");
        assert_eq!(part2(example), 24933642);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day7.txt");
        let result = part2(input);
        println!("Day 7 - Part 2: {}", result);
        assert_eq!(result, 10096985);
    }
}
