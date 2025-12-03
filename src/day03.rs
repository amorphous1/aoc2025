pub fn part1(banks: &str) -> usize {
    banks.split('\n')
        .map(|bank| { bank.chars().map(|ch| { ch.to_digit(10).unwrap() as usize }).collect() })
        .map(|bank| { max_joltage(&bank) })
        .sum()
}

fn max_joltage(bank: &Vec<usize>) -> usize {
    let (digit1, pos1) = find_max(bank, 0, bank.len()-2);
    let (digit2, _pos2) = find_max(bank, pos1+1, bank.len()-1);
    10 * digit1 + digit2
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
    }
}