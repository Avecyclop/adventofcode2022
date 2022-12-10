fn part1(input: &str) -> usize {
    let trees: Vec<Vec<u32>> = input.lines()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap()).collect()
        })
        .collect();

    let mut visibles = 2 * trees.len() + 2 * trees[0].len() - 4;
    for y in 1..trees.len() - 1 {
        for x in 1..trees[y].len() - 1 {
            let tree = trees[y][x];
            if check_up(&trees, tree, x, y - 1) ||
                check_down(&trees, tree, x, y + 1) ||
                check_left(&trees, tree, x - 1, y) ||
                check_right(&trees, tree, x + 1, y) {
                visibles += 1
            }
        }
    }
    visibles
}

fn part2(input: &str) -> i32 {
    let trees: Vec<Vec<u32>> = input.lines()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap()).collect()
        })
        .collect();

    let mut most_scenic_score = 0;
    for y in 1..trees.len() - 1 {
        for x in 1..trees[y].len() - 1 {
            let tree = trees[y][x];
            let scenic_score = scenic_up(&trees, tree, x, y - 1) *
                scenic_down(&trees, tree, x, y + 1) *
                scenic_left(&trees, tree, x - 1, y) *
                scenic_right(&trees, tree, x + 1, y);
            if scenic_score > most_scenic_score {
                most_scenic_score = scenic_score;
            }
        }
    }
    most_scenic_score
}

fn check_up(trees: &Vec<Vec<u32>>, tree: u32, x: usize, y: usize) -> bool {
    if trees[y][x] >= tree {
        return false;
    }
    return if y == 0 {
        true
    } else {
        check_up(trees, tree, x, y - 1)
    };
}

fn check_down(trees: &Vec<Vec<u32>>, tree: u32, x: usize, y: usize) -> bool {
    if trees[y][x] >= tree {
        return false;
    }
    return if y == trees.len() - 1 {
        true
    } else {
        check_down(trees, tree, x, y + 1)
    };
}

fn check_left(trees: &Vec<Vec<u32>>, tree: u32, x: usize, y: usize) -> bool {
    if trees[y][x] >= tree {
        return false;
    }
    return if x == 0 {
        true
    } else {
        check_left(trees, tree, x - 1, y)
    };
}

fn check_right(trees: &Vec<Vec<u32>>, tree: u32, x: usize, y: usize) -> bool {
    if trees[y][x] >= tree {
        return false;
    }
    return if x == trees[y].len() - 1 {
        true
    } else {
        check_right(trees, tree, x + 1, y)
    };
}

fn scenic_up(trees: &Vec<Vec<u32>>, tree: u32, x: usize, y: usize) -> i32 {
    return if trees[y][x] >= tree || y == 0 {
        1
    } else {
        1 + scenic_up(trees, tree, x, y - 1)
    }
}

fn scenic_down(trees: &Vec<Vec<u32>>, tree: u32, x: usize, y: usize) -> i32 {
    return if trees[y][x] >= tree || y == trees.len() - 1 {
        1
    } else {
        1 + scenic_down(trees, tree, x, y + 1)
    }
}

fn scenic_left(trees: &Vec<Vec<u32>>, tree: u32, x: usize, y: usize) -> i32 {
    return if trees[y][x] >= tree || x == 0 {
        1
    } else {
        1 + scenic_left(trees, tree, x - 1, y)
    }
}

fn scenic_right(trees: &Vec<Vec<u32>>, tree: u32, x: usize, y: usize) -> i32 {
    return if trees[y][x] >= tree || x == trees[y].len() - 1 {
        1
    } else {
        1 + scenic_right(trees, tree, x + 1, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("../example-input/day8.txt");
        assert_eq!(part1(example), 21);
    }

    #[test]
    fn part1_real() {
        let input = include_str!("../input/day8.txt");
        let result = part1(input);
        println!("Day 8 - Part 1: {}", result);
        assert_eq!(result, 1715);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("../example-input/day8.txt");
        assert_eq!(part2(example), 8);
    }

    #[test]
    fn part2_real() {
        let input = include_str!("../input/day8.txt");
        let result = part2(input);
        println!("Day 8 - Part 2: {}", result);
        assert_eq!(result, 374400);
    }
}
