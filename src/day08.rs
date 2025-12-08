use std::collections::HashSet;

pub fn part1(boxes: &str, num_connections: usize) -> usize {
    let boxes: Vec<Box> = boxes.lines().map(|line| Box::parse(line)).collect();
    let mut circuits = init_circuits(&boxes);
    let mut distances = init_distances(&boxes);

    for _ in 1..=num_connections {
        let (_, box1, box2) = distances.pop().unwrap();
        let c1 = circuits.iter().position(|circuit| circuit.contains(box1)).unwrap();
        if !circuits[c1].contains(box2) {
            let mut circuit1 = circuits.remove(c1);
            let c2 = circuits.iter().position(|circuit| circuit.contains(box2)).unwrap();
            let circuit2 = circuits.remove(c2);
            circuit1.extend(circuit2);
            circuits.push(circuit1);
        }
    }
    circuits.sort_by(|a,b| b.len().partial_cmp(&a.len()).unwrap());
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

pub fn part2(boxes: &str) -> usize {
    let boxes: Vec<Box> = boxes.lines().map(|line| Box::parse(line)).collect();
    let mut circuits = init_circuits(&boxes);
    let mut distances = init_distances(&boxes);

    loop {
        let (_, box1, box2) = distances.pop().unwrap();
        let c1 = circuits.iter().position(|circuit| circuit.contains(box1)).unwrap();
        if !circuits[c1].contains(box2) {
            let mut circuit1 = circuits.remove(c1);
            let c2 = circuits.iter().position(|circuit| circuit.contains(box2)).unwrap();
            let circuit2 = circuits.remove(c2);
            circuit1.extend(circuit2);
            circuits.push(circuit1);
            if circuits.len() == 1 {
                return (box1.x * box2.x) as usize;
            }
        }
    }
}

fn init_circuits(boxes: &Vec<Box>) -> Vec<HashSet<Box>> {
    let mut circuits: Vec<HashSet<Box>> = Vec::new();
    for b in boxes {
        let mut circuit: HashSet<Box> = HashSet::new();
        circuit.insert(b.clone());
        circuits.push(circuit);
    }
    circuits
}

fn init_distances(boxes: &Vec<Box>) -> Vec<(f64, &Box, &Box)> {
    let mut distances: Vec<(f64, &Box, &Box)> = Vec::new();
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            let distance = boxes[i].distance(&boxes[j]);
            distances.push((distance, &boxes[i], &boxes[j]));
        }
    }
    distances.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    distances
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Box {
    fn parse(text: &str) -> Box {
        let coords: Vec<i64> = text.split(',').map(|v| v.parse().unwrap()).collect();
        Box { x: coords[0], y: coords[1], z: coords[2] }
    }

    fn distance(&self, other: &Box) -> f64 {
        (((other.x-self.x)*(other.x-self.x) + (other.y-self.y)*(other.y-self.y) + (other.z-self.z)*(other.z-self.z)) as f64).sqrt()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn samples() {
        let sample_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        assert_eq!(super::part1(sample_input, 10), 40);
        assert_eq!(super::part2(sample_input), 25272);
    }
}