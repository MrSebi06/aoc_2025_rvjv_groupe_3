pub fn calculate_squared_distance(from: &[f32; 3], to: &[f32; 3]) -> f32 {
    ((from[0]-to[0])*(from[0]-to[0])) + ((from[1]-to[1])*(from[1]-to[1])) + ((from[2]-to[2])*(from[2]-to[2]))
}

pub fn find_circuit_index_from_junction_box_if_exist(
    junction_box: &[f32; 3],
    circuits: &[Vec<[f32; 3]>]
) -> Option<usize> {
    circuits.iter().position(|circuit| {
        circuit.contains(junction_box)
    })
}

pub fn p1(input: &str, max_connections: usize) -> u64 {
    let mut count: u64 = 1;

    let mut circuits: Vec<Vec<[f32; 3]>> = Vec::new();
    let mut junction_boxes: Vec<[f32; 3]> = Vec::new();

    for line in input.lines() {
        let mut xyz = line.split(',');

        let x = xyz.next().unwrap().parse::<f32>().unwrap();
        let y = xyz.next().unwrap().parse::<f32>().unwrap();
        let z = xyz.next().unwrap().parse::<f32>().unwrap();

        junction_boxes.push([x, y, z]);
    }

    let mut edges: Vec<(f32, [f32; 3], [f32; 3])> = Vec::new();
    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let squared_distance = calculate_squared_distance(&junction_boxes[i], &junction_boxes[j]);
            edges.push((squared_distance, junction_boxes[i], junction_boxes[j]));
        }
    }
    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for connection in 0..max_connections {
        let (_, junction_box_a, junction_box_b) = edges[connection];

        let index_of_the_circuit_of_junction_box_a = find_circuit_index_from_junction_box_if_exist(&junction_box_a, &circuits);
        let index_of_the_circuit_of_junction_box_b = find_circuit_index_from_junction_box_if_exist(&junction_box_b, &circuits);

        match (index_of_the_circuit_of_junction_box_a, index_of_the_circuit_of_junction_box_b) {
            // Same circuit, we don't add/create new one
            (Some(index_circuit_a), Some(index_circuit_b)) if index_circuit_a == index_circuit_b => {},
            // Here is the trick of this day event, the two circuits are different, we need to merge them
            (Some(index_circuit_a), Some(index_circuit_b)) => {
                // We remove the circuit with the bigger index to avoid extend the smaller one
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

pub fn p2(input: &str) -> u64 {

    /*
    for line in input.lines() {
        for (i, cur_space) in line.chars().enumerate() {


        }

    }
    */

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::d8::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d8.txt");
        assert_eq!(p1(input, 1000), 40)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d8_test.txt");
        assert_eq!(p2(input), 40)
    }
}
