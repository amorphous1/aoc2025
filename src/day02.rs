pub fn part1(input: &str) -> u64 {
    let mut result = 0;
    input.split(',').for_each(|range| {
        let mut from_to = range.split('-');
        let from: u64 = from_to.next().unwrap().parse().unwrap();
        let to: u64 = from_to.next().unwrap().parse().unwrap();
        for num in from..=to {
            let num_text = num.to_string();
            if num_text.len() % 2 == 0 && num_text[..num_text.len() / 2] == num_text[num_text.len() / 2..] {
                result += num;
            }
        }
    });
    result
}

pub fn part2(input: &str) -> u64 {
    let mut result = 0;
    input.split(',').for_each(|range| {
        let mut from_to = range.split('-');
        let from: u64 = from_to.next().unwrap().parse().unwrap();
        let to: u64 = from_to.next().unwrap().parse().unwrap();
        for num in from..=to {
            let num_text = num.to_string();
            if is_repeated(num_text.as_str()) {
                result += num;
            }
        }
    });
    result
}

fn is_repeated(num_text: &str) -> bool {
    for n in 2..=num_text.len() {
        if is_repeated_n_times(num_text, &n) {
            return true;
        }
    }
    false
}

fn is_repeated_n_times(num_text: &str, n: &usize) -> bool {
    if num_text.len() % n != 0 {
        return false;
    }
    let part_len = num_text.len() / n;
    for i in 0..n-1 {
        if num_text[i*part_len..(i+1)*part_len] != num_text[(i+1)*part_len..(i+2)*part_len] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(part1(sample_input), 1227775554);
        assert_eq!(part2(sample_input), 4174379265);
    }
}