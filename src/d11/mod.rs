use itertools::Itertools;
use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

pub fn p1(input: &str) -> usize {
    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (key, values) = line.split_once(':').expect("invalid");
            (key, values.split_whitespace().collect())
        })
        .chain(std::iter::once(("out", vec![])))
        .collect();

    let source = "you";
    let destination = "out";

    let mut indegree: HashMap<&str, usize> = graph.iter().map(|(&k, _)| (k, 0)).collect();

    for (_, children) in graph.iter() {
        for child in children {
            *indegree.get_mut(child).expect("unknown node") += 1;
        }
    }

    // Perform topological sort using Kahn's algorithm
    let mut queue: Vec<&str> = indegree
        .iter()
        .filter(|(_, v)| **v == 0usize)
        .map(|(&k, _)| k)
        .collect();

    let mut topo_order: Vec<&str> = vec![];
    while !queue.is_empty() {
        let node = queue.remove(0);
        topo_order.push(node);
        for &neighbor in graph[node].iter() {
            *indegree.get_mut(neighbor).expect("unknown node") -= 1;
            if indegree[neighbor] == 0 {
                queue.push(neighbor);
            }
        }
    }

    let mut ways: HashMap<&str, usize> = graph.iter().map(|(&k, _)| (k, 0)).collect();
    *ways.get_mut(source).expect("unknown node") = 1;

    for node in topo_order {
        for &neighbor in graph[node].iter() {
            *ways.get_mut(neighbor).expect("unknown node") += ways[node];
        }
    }

    ways[destination]
}

pub fn p2(input: &str) -> usize {
    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (key, values) = line.split_once(':').expect("invalid");
            (key, values.split_whitespace().collect())
        })
        .chain(std::iter::once(("out", vec![])))
        .collect();

    let source = "svr";
    let intermediary = ["dac", "fft"];
    let destination = "out";

    let mut indegree: HashMap<&str, usize> = graph.iter().map(|(&k, _)| (k, 0)).collect();

    for (_, children) in graph.iter() {
        for child in children {
            *indegree.get_mut(child).expect("unknown node") += 1;
        }
    }

    // Perform topological sort using Kahn's algorithm
    let mut queue: Vec<&str> = indegree
        .iter()
        .filter(|(_, v)| **v == 0usize)
        .map(|(&k, _)| k)
        .collect();

    let mut topo_order: Vec<&str> = vec![];
    while !queue.is_empty() {
        let node = queue.remove(0);
        topo_order.push(node);
        for &neighbor in graph[node].iter() {
            *indegree.get_mut(neighbor).expect("unknown node") -= 1;
            if indegree[neighbor] == 0 {
                queue.push(neighbor);
            }
        }
    }

    let count = intermediary
        .iter()
        .permutations(intermediary.len())
        .map(|perm| {
            let waypoints: Vec<&str> = std::iter::once(source)
                .chain(perm.iter().map(|&&s| s))
                .chain(std::iter::once(destination))
                .collect();
            waypoints
                .windows(2)
                .map(|w| {
                    let (from, to) = (w[0], w[1]);
                    let mut ways: HashMap<&str, usize> =
                        graph.iter().map(|(&k, _)| (k, 0)).collect();
                    *ways.get_mut(from).expect("unknown node") = 1;
                    for &node in topo_order.iter() {
                        for &neighbor in graph[node].iter() {
                            *ways.get_mut(neighbor).expect("unknown node") += ways[node];
                        }
                    }
                    ways[to]
                })
                .product::<usize>()
        })
        .sum();

    count
}

#[cfg(test)]
mod tests {
    use crate::d11::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d11_test.txt");
        assert_eq!(p1(input), 5)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d11_test_p2.txt");
        assert_eq!(p2(input), 2)
    }
}
