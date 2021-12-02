use crypto::md5::Md5;
use crypto::digest::Digest;

use rayon::prelude::*;

const MAX_N: usize = 10_000_000;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> String {
    input.trim().to_owned()
}

fn md5_hash_starts_with(prefix: &str, i: usize, expect: &str) -> bool {
    let mut sh = Md5::new();
    let s = format!("{}{}", prefix, i);
    sh.input_str(&s);
    sh.result_str().starts_with(expect)
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    (0..MAX_N).into_par_iter().find_first(|&i| md5_hash_starts_with(input, i, "00000")).unwrap_or(0)
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    (0..MAX_N).into_par_iter().find_first(|&i| md5_hash_starts_with(input, i, "000000")).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        assert_eq!(part1("abcdef"), 609043)
    }

    #[test]
    fn part1_ex2() {
        assert_eq!(part1("pqrstuv"), 1048970)
    }

    // #[test]
    // fn part2_ex1() {
    //     assert_eq!(part2(&input_generator(EXAMPLE_INPUT)), 6)
    // }
}
