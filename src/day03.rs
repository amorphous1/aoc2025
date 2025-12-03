pub fn part1(banks: &str) -> usize {
    banks.split('\n')
        .map(|bank| { bank.chars().map(|ch| { ch.to_digit(10).unwrap() as usize }).collect() })
        .map(|bank| { max_joltage(&bank) })
        .sum()
}

pub fn part2(banks: &str) -> usize {
    banks.split('\n')
        .map(|bank| { bank.chars().map(|ch| { ch.to_digit(10).unwrap() as usize }).collect() })
        .map(|bank| { max_joltage12(&bank) })
        .sum()
}
fn max_joltage(bank: &Vec<usize>) -> usize {
    let (digit1, pos1) = find_max(bank, 0, bank.len()-2);
    let (digit2, _pos2) = find_max(bank, pos1+1, bank.len()-1);
    10 * digit1 + digit2
}

fn max_joltage12(bank: &Vec<usize>) -> usize {
    let (digit1, pos1) = find_max(bank, 0, bank.len()-12);
    let (digit2, pos2) = find_max(bank, pos1+1, bank.len()-11);
    let (digit3, pos3) = find_max(bank, pos2+1, bank.len()-10);
    let (digit4, pos4) = find_max(bank, pos3+1, bank.len()-9);
    let (digit5, pos5) = find_max(bank, pos4+1, bank.len()-8);
    let (digit6, pos6) = find_max(bank, pos5+1, bank.len()-7);
    let (digit7, pos7) = find_max(bank, pos6+1, bank.len()-6);
    let (digit8, pos8) = find_max(bank, pos7+1, bank.len()-5);
    let (digit9, pos9) = find_max(bank, pos8+1, bank.len()-4);
    let (digit10, pos10) = find_max(bank, pos9+1, bank.len()-3);
    let (digit11, pos11) = find_max(bank, pos10+1, bank.len()-2);
    let (digit12, _pos12) = find_max(bank, pos11+1, bank.len()-1);
    100000000000*digit1 + 10000000000*digit2 + 1000000000*digit3 + 100000000*digit4 + 10000000*digit5 + 1000000*digit6 + 100000*digit7 + 10000*digit8 + 1000*digit9 + 100*digit10 + 10*digit11 + digit12
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