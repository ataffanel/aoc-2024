static INPUT: &'static str = include_str!("input.txt");

fn decode_data(input: impl AsRef<str>) -> Vec<Vec<i64>> {
    input
        .as_ref()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn part_1(input: impl AsRef<str>) -> i64 {
    let data = decode_data(input);

    data.iter()
        .map(|line| {
            let difference = line.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

            difference.iter().all(|d| (*d < 0) && (d.abs() <= 3))
                || difference.iter().all(|d| (*d > 0) && (d.abs() <= 3))
        })
        .filter(|v| *v)
        .count() as i64
}

fn part_2(_input: impl AsRef<str>) -> i64 {
    todo!()
}

fn main() {
    println!("Safe reports: {}", part_1(INPUT));
    println!("Safe report with dampening: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_1() {
        let expected = 2;

        let actual = part_1(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let expected = 4;

        let actual = part_2(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
