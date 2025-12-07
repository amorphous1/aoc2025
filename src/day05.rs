pub fn part1(problems: &str) -> usize {
    let (numbers, operators) = problems.rsplit_once('\n').unwrap();
    let numbers: Vec<Vec<usize>> = numbers.lines()
        .map(|line| line.split_ascii_whitespace().map(|number| number.parse().unwrap()).collect())
        .collect();
    let operators: Vec<char> = operators.split_ascii_whitespace().map(|op| op.chars().next().unwrap()).collect();

    let mut result = 0;
    for x in 0..numbers[0].len() {
        let mut problem_result = if operators[x] == '+' { 0 } else { 1 };
        for y in 0..numbers.len() {
            if operators[x] == '+' { problem_result += numbers[y][x] } else { problem_result *= numbers[y][x] };
        }
        result += problem_result;
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples() {
        let sample_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";

        assert_eq!(super::part1(sample_input), 4277556);
        // assert_eq!(super::part2(sample_input), 43);
    }
}