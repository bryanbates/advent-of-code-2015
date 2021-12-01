type Gift = (u32, u32, u32);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut lwh = l
                .trim()
                .split('x')
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            // Useful for these to be sorted ascending.
            lwh.sort_unstable();
            (lwh[0], lwh[1], lwh[2])
        })
        .collect()
}

fn wrap_gift(g: Gift) -> u32 {
    let s1 = g.0 * g.1;
    let s2 = g.1 * g.2;
    let s3 = g.2 * g.0;

    2 * (s1 + s2 + s3) + s1
}

#[aoc(day2, part1)]
pub fn part1(input: &[Gift]) -> u32 {
    input.iter().map(|g| wrap_gift(*g)).sum()
}

fn ribbon_gift(g: Gift) -> u32 {
    let bow = g.0 * g.1 * g.2;
    let perimeter = 2 * g.0 + 2 * g.1;
    perimeter + bow
}

#[aoc(day2, part2)]
pub fn part2(input: &[Gift]) -> u32 {
    input.iter().map(|g| ribbon_gift(*g)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1(&input_generator("2x3x4")), 58);
    }
    #[test]
    fn part1_ex2() {
        assert_eq!(part1(&input_generator("1x1x10")), 43);
    }

    #[test]
    fn part2_ex1() {
        assert_eq!(part2(&input_generator("2x3x4")), 34);
    }
    #[test]
    fn part2_ex2() {
        assert_eq!(part2(&input_generator("1x1x10")), 14);
    }
}
