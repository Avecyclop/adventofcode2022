fn part1(input: &str) -> i32 {
    let instructions: Vec<i32> = input.lines()
        .map(|line| if line.eq("noop") { 0 } else { line.split(" ").last().unwrap().parse::<i32>().unwrap() })
        .collect();

    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut x = 1;
    let mut i = 0;
    let mut cycle = 0;
    while cycle <= 220 {
        cycle += 1;
        if cycle % 40 == 20 {
            signal_strengths.push(cycle * x);
        }
        let instruction = instructions[i as usize % instructions.len()];
        if instruction != 0 {
            cycle += 1;
            if cycle % 40 == 20 {
                signal_strengths.push(cycle * x);
            }
            x += instruction;
        }
        i += 1;
    }
    signal_strengths.iter().sum()
}

fn part2(input: &str) -> String {
    let instructions: Vec<i32> = input.lines()
        .map(|line| if line.eq("noop") { 0 } else { line.split(" ").last().unwrap().parse::<i32>().unwrap() })
        .collect();

    let mut crt: Vec<Vec<char>> = vec![vec!['.'; 40]; 6];
    let mut x: i32 = 1;
    let mut i = 0;
    let mut cycle = 0;
    while cycle < 240 {
        if x.abs_diff(cycle % 40) <= 1 {
            crt[cycle as usize / 40][cycle as usize % 40] = '#';
        }
        cycle += 1;
        let instruction = instructions[i as usize % instructions.len()];
        if instruction != 0 {
            if x.abs_diff(cycle % 40) <= 1 {
                crt[cycle as usize / 40][cycle as usize % 40] = '#';
            }
            cycle += 1;
            x += instruction;
        }
        i += 1;
    }
    crt.iter()
        .map(|row| { row.iter().collect::<String>() })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day10.txt");
        assert_eq!(part1(example), 13140);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day10.txt");
        let result = part1(input);
        println!("Day 10 - Part 1: {}", result);
        assert_eq!(result, 13860);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day10.txt");
        let result = part2(example);
        println!("{}", result);
        assert_eq!(result, "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day10.txt");
        let result = part2(input);
        println!("Day 10 - Part 2: \n{}", result);
        assert_eq!(result, "\
###..####.#..#.####..##....##..##..###..
#..#....#.#..#.#....#..#....#.#..#.#..#.
#..#...#..####.###..#.......#.#....###..
###...#...#..#.#....#.##....#.#....#..#.
#.#..#....#..#.#....#..#.#..#.#..#.#..#.
#..#.####.#..#.#.....###..##...##..###.."); //RZHFGJCB
    }
}
