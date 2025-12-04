pub fn part1(diagram: &str) -> usize {
    let grid: Vec<Vec<char>> = diagram.split('\n').map(|line| line.chars().collect()).collect();
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' && num_neighbours(&grid, x, y) < 4 {
                result += 1;
            }
        }
    }
    result
}

fn num_neighbours(grid: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut result = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if (dx != 0 || dy != 0) && nx >= 0 && nx < grid[0].len() as i32 && ny >= 0 && ny < grid.len() as i32 && grid[ny as usize][nx as usize] == '@' {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples() {
        let sample_input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(super::part1(sample_input), 13);
        // assert_eq!(super::part2(sample_input), 3121910778619);
    }
}