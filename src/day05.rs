use rangemap::RangeInclusiveSet;

pub fn part1(database: &str) -> usize {
    let mut db_iter = database.splitn(2, "\n\n");
    let (db_ranges, db_ids) = (db_iter.next().unwrap(), db_iter.next().unwrap());
    let mut fresh_ingredients: RangeInclusiveSet<usize> = RangeInclusiveSet::new();
    db_ranges.lines().for_each(|line| {
        let mut range_iter = line.split("-");
        let (from, to): (usize, usize) = (range_iter.next().unwrap().parse().unwrap(), range_iter.next().unwrap().parse().unwrap());
        fresh_ingredients.insert(from..=to);
    });
    let db_ids = db_ids.split('\n').map(|line| line.parse::<usize>().unwrap());
    db_ids.filter(|&id| fresh_ingredients.contains(&id)).count()
}

pub fn part2(database: &str) -> usize {
    let db_ranges = database.split("\n\n").next().unwrap();
    let mut fresh_ingredients: RangeInclusiveSet<usize> = RangeInclusiveSet::new();
    db_ranges.lines().for_each(|line| {
        let mut range_iter = line.split("-");
        let (from, to): (usize, usize) = (range_iter.next().unwrap().parse().unwrap(), range_iter.next().unwrap().parse().unwrap());
        fresh_ingredients.insert(from..=to);
    });
    fresh_ingredients.iter().map(|range| range.end() - range.start() + 1).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples() {
        let sample_input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(super::part1(sample_input), 3);
        assert_eq!(super::part2(sample_input), 14);
    }
}