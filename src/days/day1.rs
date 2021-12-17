use std::error::Error;

use crate::shared::read_file;

pub fn run() -> Result<(usize, usize), Box<dyn Error>> {
    let path = "src/input/day_1.txt";
    let contents = read_file::<usize>(path, '\n')?;

    let part_1 = part_1(&contents);
    let part_2 = part_2(&contents);

    Ok((part_1, part_2))
}

pub fn input_generator(input: &str) -> Vec<usize> {
    input.split('\n').map(|val| val.parse().unwrap()).collect()
}

pub fn part_1(input: &[usize]) -> usize {
    let output = count_elevation_changes(input);

    output
}

pub fn part_2(input: &[usize]) -> usize {
    let condensed_list: Vec<usize> = input
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect();

    let output = count_elevation_changes(&condensed_list);

    output
}

fn count_elevation_changes(input: &[usize]) -> usize {
    input.windows(2).map(|val| (val[0] < val[1]) as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part_1, part_2};

    const INPUT: &'static str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn p1() {
        let output = part_1(&input_generator(&INPUT));
        assert_eq!(output, 7);
    }

    #[test]
    fn p2() {
        let output = part_2(&input_generator(&INPUT));
        assert_eq!(output, 5);
    }
}
