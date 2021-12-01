#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .trim()
        .chars()
        .map(|ch| match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut total = 0;
    for (i, val) in (0..).zip(input.trim().chars().map(|ch| match ch {
        '(' => 1,
        ')' => -1,
        _ => 0,
    })) {
        total += val;
        if total == -1 {
            return i + 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    #[test]
    fn part1_ex2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    #[test]
    fn part1_ex3() {
        assert_eq!(part1("))((((("), 3);
    }

    #[test]
    fn part1_ex4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    #[test]
    fn part1_ex5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(")"), 1);
    }

    #[test]
    fn part2_ex2() {
        assert_eq!(part2("()())"), 5);
    }
}
