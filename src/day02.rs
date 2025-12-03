pub fn part1(input: &str) -> u64 {
    find_repeating_numbers(input, &is_repeated_two_times)
}

pub fn part2(input: &str) -> u64 {
    find_repeating_numbers(input, &is_repeated_any_number_of_times)
}

fn find_repeating_numbers(ranges: &str, is_repeated: &dyn Fn(&str) -> bool) -> u64 {
    let mut result = 0;
    ranges.split(',').for_each(|range| {
        let mut from_to = range.split('-');
        let from: u64 = from_to.next().unwrap().parse().unwrap();
        let to: u64 = from_to.next().unwrap().parse().unwrap();
        for num in from..=to {
            if is_repeated(num.to_string().as_str()) {
                result += num;
            }
        }
    });
    result
}

fn is_repeated_two_times(num_text: &str) -> bool {
    is_repeated_n_times(num_text, &2)
}

fn is_repeated_any_number_of_times(num: &str) -> bool {
    for n in 2..=num.len() {
        if is_repeated_n_times(num, &n) {
            return true;
        }
    }
    false
}

fn is_repeated_n_times(num: &str, n: &usize) -> bool {
    if num.len() % n != 0 {
        return false;
    }
    let part_len = num.len() / n;
    for i in 0..n-1 {
        if num[i*part_len..(i+1)*part_len] != num[(i+1)*part_len..(i+2)*part_len] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples() {
        let sample_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(super::part1(sample_input), 1227775554);
        assert_eq!(super::part2(sample_input), 4174379265);
    }
}