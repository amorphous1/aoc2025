pub fn part1(banks: &str) -> usize {
    total_joltage(banks, 2)
}

pub fn part2(banks: &str) -> usize {
    total_joltage(banks, 12)
}

fn total_joltage(banks: &str, num_batteries: usize) -> usize {
    banks.split('\n')
        .map(|bank| { bank.chars().map(|ch| { ch.to_digit(10).unwrap() as usize }).collect() })
        .map(|bank| { max_joltage(&bank, num_batteries) })
        .sum()

}

fn max_joltage(bank: &Vec<usize>, num_batteries: usize) -> usize {
    let mut result: usize = 0;
    let mut from = 0;
    for to in bank.len()-num_batteries..=bank.len()-1 {
        result *= 10;
        let (digit, pos) = find_max(bank, from, to);
        result += digit;
        from = pos + 1;
    }
    result
}

fn find_max(bank: &Vec<usize>, from: usize, to: usize) -> (usize, usize) {
    for digit in (1..=9).rev() {
        for pos in from..=to {
            if bank[pos] == digit {
                return (digit, pos);
            }
        }
    }
    (bank[from], from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples() {
        let sample_input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(super::part1(sample_input), 357);
        assert_eq!(super::part2(sample_input), 3121910778619);
    }
}