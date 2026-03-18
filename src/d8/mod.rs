pub fn calculate_squared_distance(from: &[i64; 3], to: &[i64; 3]) -> i64 {
    ((from[0]-to[0])*(from[0]-to[0])) + ((from[1]-to[1])*(from[1]-to[1])) + ((from[2]-to[2])*(from[2]-to[2]))
}

pub fn find_circuit_index_from_junction_box_if_exist(
    junction_box: &[i64; 3],
    circuits: &[Vec<[i64; 3]>]
) -> Option<usize> {
    circuits.iter().position(|circuit| {
        circuit.contains(junction_box)
    })
}

pub fn p1(input: &str, max_connections: usize) -> u64 {
    let mut count: u64 = 1;

    let mut circuits: Vec<Vec<[i64; 3]>> = Vec::new();
    let mut junction_boxes: Vec<[i64; 3]> = Vec::new();

    for line in input.lines() {
        let mut xyz = line.split(',');

        let x = xyz.next().unwrap().parse::<i64>().unwrap();
        let y = xyz.next().unwrap().parse::<i64>().unwrap();
        let z = xyz.next().unwrap().parse::<i64>().unwrap();

        junction_boxes.push([x, y, z]);
    }

    let mut edges: Vec<(i64, [i64; 3], [i64; 3])> = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let squared_distance = calculate_squared_distance(&junction_boxes[i], &junction_boxes[j]);
            edges.push((squared_distance, junction_boxes[i], junction_boxes[j]));
        }
    }
    edges.sort_by(|a, b| a.0.cmp(&b.0));

    for connection in 0..max_connections {
        let (_, junction_box_a, junction_box_b) = edges[connection];

        let index_of_the_circuit_of_junction_box_a = find_circuit_index_from_junction_box_if_exist(&junction_box_a, &circuits);
        let index_of_the_circuit_of_junction_box_b = find_circuit_index_from_junction_box_if_exist(&junction_box_b, &circuits);

        match (index_of_the_circuit_of_junction_box_a, index_of_the_circuit_of_junction_box_b) {
            // Same circuit, we don't add/create new one
            (Some(index_circuit_a), Some(index_circuit_b)) if index_circuit_a == index_circuit_b => {},
            // Here is the trick of this day event, the two circuits are different, we need to merge them
            (Some(index_circuit_a), Some(index_circuit_b)) => {
                // We remove the circuit at the larger index first so the smaller index stays valid
                let larger_index = index_circuit_a.max(index_circuit_b);
                let smaller_index = index_circuit_a.min(index_circuit_b);

                let removed_circuit = circuits.remove(larger_index);
                circuits[smaller_index].extend(removed_circuit);
            },
            // Only one is in the circuit, we add the other
            (Some(index_circuit_a), None) => { circuits[index_circuit_a].push(junction_box_b); },
            (None, Some(index_circuit_b)) => { circuits[index_circuit_b].push(junction_box_a); },
            // Nones, so we create the new circuit
            (None, None) => { circuits.push(vec![junction_box_a, junction_box_b]); },
        }
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    for i in 0..3 {
        count *= circuits[i].len() as u64;
    }

    count
}

pub fn p2(input: &str) -> i64 {
    let mut circuits: Vec<Vec<[i64; 3]>> = Vec::new();
    let mut junction_boxes: Vec<[i64; 3]> = Vec::new();

    let mut last_junction_box_a: [i64; 3] = [0, 0, 0];
    let mut last_junction_box_b: [i64; 3] = [0, 0, 0];

    for line in input.lines() {
        let mut xyz = line.split(',');

        let x = xyz.next().unwrap().parse::<i64>().unwrap();
        let y = xyz.next().unwrap().parse::<i64>().unwrap();
        let z = xyz.next().unwrap().parse::<i64>().unwrap();

        junction_boxes.push([x, y, z]);
    }

    let mut edges: Vec<(i64, [i64; 3], [i64; 3])> = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let squared_distance = calculate_squared_distance(&junction_boxes[i], &junction_boxes[j]);
            edges.push((squared_distance, junction_boxes[i], junction_boxes[j]));
        }
    }
    edges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut i = 0;

    while circuits.is_empty() || circuits.len() != 1 || circuits[0].len() != junction_boxes.len() {
        let (_, junction_box_a, junction_box_b) = edges[i];

        let index_of_the_circuit_of_junction_box_a = find_circuit_index_from_junction_box_if_exist(&junction_box_a, &circuits);
        let index_of_the_circuit_of_junction_box_b = find_circuit_index_from_junction_box_if_exist(&junction_box_b, &circuits);

        match (index_of_the_circuit_of_junction_box_a, index_of_the_circuit_of_junction_box_b) {
            // Same circuit, we don't add/create new one
            (Some(index_circuit_a), Some(index_circuit_b)) if index_circuit_a == index_circuit_b => {},
            // Here is the trick of this day event, the two circuits are different, we need to merge them
            (Some(index_circuit_a), Some(index_circuit_b)) => {
                last_junction_box_a = junction_box_a;
                last_junction_box_b = junction_box_b;

                // We remove the circuit at the larger index first so the smaller index stays valid
                let larger_index = index_circuit_a.max(index_circuit_b);
                let smaller_index = index_circuit_a.min(index_circuit_b);

                let removed_circuit = circuits.remove(larger_index);
                circuits[smaller_index].extend(removed_circuit);
            },
            // Only one is in the circuit, we add the other
            (Some(index_circuit_a), None) => {
                last_junction_box_a = junction_box_a;
                last_junction_box_b = junction_box_b;
                circuits[index_circuit_a].push(junction_box_b);
            },
            (None, Some(index_circuit_b)) => {
                last_junction_box_a = junction_box_a;
                last_junction_box_b = junction_box_b;
                circuits[index_circuit_b].push(junction_box_a);
            },
            // Nones, so we create the new circuit
            (None, None) => { circuits.push(vec![junction_box_a, junction_box_b]); },
        }

        i += 1;
    }

    println!("Len circuits: {}", circuits.len());
    println!("last_junction_box_a: x:{},y:{},z{}", last_junction_box_a[0], last_junction_box_a[1], last_junction_box_a[2]);
    println!("last_junction_box_b: x:{},y:{},z{}", last_junction_box_b[0], last_junction_box_b[1], last_junction_box_b[2]);
    last_junction_box_a[0] * last_junction_box_b[0]
}

#[cfg(test)]
mod tests {
    use crate::d8::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d8_test.txt");
        assert_eq!(p1(input, 10), 40)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d8_test.txt");
        assert_eq!(p2(input), 25272)
    }
}
