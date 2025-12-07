use std::collections::HashSet;

pub fn part1(diagram: &str) -> usize {
    let diagram: Vec<Vec<char>> = diagram.lines().map(|line| line.chars().collect()).collect();
    let mut num_splits = 0;

    let mut beams_x: HashSet<usize> = HashSet::new();
    beams_x.insert(diagram[0].iter().position(|&c| c == 'S').unwrap());

    for y in 1..diagram.len() {
        let mut new_beams_x: HashSet<usize> = HashSet::new();
        for beam_x in &beams_x {
            if diagram[y][*beam_x] == '^' {
                new_beams_x.insert(*beam_x - 1);
                new_beams_x.insert(*beam_x + 1);
                num_splits += 1;
            } else {
                new_beams_x.insert(*beam_x);
            }
        }
        beams_x = new_beams_x;
    }

    num_splits
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples() {
        let sample_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        assert_eq!(super::part1(sample_input), 21);
        // assert_eq!(super::part2(sample_input), 3263827);
    }
}