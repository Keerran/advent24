use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip()
}

pub fn part1(input: &str) -> Option<u32> {
    let (mut lhs, mut rhs) = parse_input(input);
    lhs.sort();
    rhs.sort();
    Some(lhs.into_iter().zip(rhs).fold(0, |acc, (a, b)| acc + a.abs_diff(b)))
}

pub fn part2(input: &str) -> Option<u32> {
    let (lhs, rhs) = parse_input(input);

    let mut counter = HashMap::new();
    for num in rhs {
        *counter.entry(num).or_insert(0) += 1;
    }

    Some(lhs.into_iter().fold(0, |acc, n| acc + n * counter.get(&n).unwrap_or(&0)))
}

fn main() {
    let input = crusty::get_input("inputs", 1);
    crusty::display(1, part1(&input));
    crusty::display(2, part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = crusty::get_input("samples", 1);
        assert_eq!(part1(&input), Some(11));
    }

    #[test]
    fn test_part2() {
        let input = crusty::get_input("samples", 1);
        assert_eq!(part2(&input), Some(31));
    }
}
