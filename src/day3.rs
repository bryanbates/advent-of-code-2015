use std::collections::HashSet;

type Point = (i32, i32);

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut houses = HashSet::<Point>::new();

    houses.insert((x, y));

    for dir in input.chars() {
        match dir {
            '^' => y += 1,
            '>' => x += 1,
            '<' => x -= 1,
            'v' => y -= 1,
            _ => {}
        }
        houses.insert((x, y));
    }

    houses.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut rx: i32 = 0;
    let mut ry: i32 = 0;

    let mut houses = HashSet::<Point>::new();

    houses.insert((x, y));
    houses.insert((rx, ry));

    let mut dirs = input.chars();

    while let Some(dir) = dirs.next() {
        match dir {
            '^' => y += 1,
            '>' => x += 1,
            '<' => x -= 1,
            'v' => y -= 1,
            _ => {}
        }
        if let Some(rdir) = dirs.next() {
            match rdir {
                '^' => ry += 1,
                '>' => rx += 1,
                '<' => rx -= 1,
                'v' => ry -= 1,
                _ => {}
            }
        }
        houses.insert((x, y));
        houses.insert((rx, ry));
    }

    houses.len()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(">"), 2)
    }

    #[test]
    fn part1_ex2() {
        assert_eq!(part1("^>v<"), 4)
    }

    #[test]
    fn part1_ex3() {
        assert_eq!(part1("^v^v^v^v^v"), 2)
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2("^v"), 3)
    }

    #[test]
    fn part2_ex2() {
        assert_eq!(part2("^>v<"), 3)
    }

    #[test]
    fn part2_ex3() {
        assert_eq!(part2("^v^v^v^v^v"), 11)
    }

    // #[test]
    // fn part2_ex1() {
    //     assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 6)
    // }
}
