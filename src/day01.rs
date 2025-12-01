pub fn part1(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut result: u32 = 0;
    input.split('\n').for_each(|line| {
        let mut rotation = line.to_string();
        let direction = rotation.remove(0);
        let rotation: i32 = rotation.parse().unwrap();
        if direction == 'R' {
            position = (position + rotation) % 100;
        } else {
            position = (position - rotation) % 100;
        }
        if position < 0 {
            position += 100;
        }
        if position == 0 {
            result += 1;
        }
    });
    result
}

pub fn part2(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut result: u32 = 0;
    input.split('\n').for_each(|line| {
        let mut rotation = line.to_string();
        let direction = rotation.remove(0);
        let rotation: u32 = rotation.parse().unwrap();
        result += rotation / 100;
        let rotation: i32 = (rotation % 100).try_into().unwrap();
        let rotation: i32 = if direction == 'R' { rotation } else { -rotation };

        let new_position = (((position + rotation) % 100) + 100) % 100;
        if new_position < position && rotation > 0 || new_position > position && position > 0 && rotation  < 0 || new_position == 0 {
            result += 1;
        }
        position = new_position;
    });
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let sample_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part1(sample_input), 3);
        assert_eq!(part2(sample_input), 6);
    }
}