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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(part1(sample_input), 1227775554);
    }
}