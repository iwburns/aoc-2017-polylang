fn main() {
    let input = include_str!("./input.txt");
    println!("{}", compute_checksum(input));
}

fn compute_checksum(input: &str) -> u32 {
    input.lines().map(|line| get_line_diff(line)).sum()
}

fn get_line_diff(line: &str) -> u32 {
    let numbers: Vec<u32> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap_or(0))
        .collect();

    let min = numbers.iter().min().unwrap_or(&0);
    let max = numbers.iter().max().unwrap_or(&0);
    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input =
              "5 1 9 5
             \n7 5 3\
             \n2 4 6 8";

        assert_eq!(compute_checksum(input), 18);
    }
}
