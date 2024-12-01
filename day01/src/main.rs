
static INPUT: &'static str = include_str!("input.txt");

fn decode_data(input: impl AsRef<str>) -> (Vec<i64>, Vec<i64>) {

    let map: Vec<Vec<i64>> = input.as_ref().lines().map(|line| {
        line.split_whitespace().map(str::parse::<i64>).map(Result::unwrap).collect()
    }).collect();

    let map_left = map.iter().map(|m| m[0]).collect::<Vec<_>>();
    let map_right = map.iter().map(|m| m[1]).collect::<Vec<_>>();

    (map_left, map_right)
}

fn part_1(input: impl AsRef<str>) -> i64 {
    let (mut map_left, mut map_right) = decode_data(input);

    map_left.sort();
    map_right.sort();

    map_left.iter().zip(map_right).map(|(l, r)| (r - l).abs()).sum()
}

fn part_2(input: impl AsRef<str>) -> i64 {
    let (map_left, map_right) = decode_data(input);

    map_left.iter().map(|left| {
        let appearance = map_right.iter().filter(|right| left == *right).count();
        left * appearance as i64
    }).sum()
}

fn main() {
    let sum = part_1(INPUT);
    println!("Difference is {}", sum);
    let similarity = part_2(INPUT);
    println!("Similarity is: {}", similarity);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        let expected = 11;

        let actual = part_1(EXAMPLE);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let expected = 31;

        let actual = part_2(EXAMPLE);

        assert_eq!(expected, actual);
    }
}
