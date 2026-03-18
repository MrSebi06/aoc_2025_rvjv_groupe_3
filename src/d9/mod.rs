use std::cmp::max;
use std::io::Split;

pub fn p1(input: &str) -> u64 {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|x| {
            let mut split = x.splitn(2, ',');
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let mut max_area = 0;
    for i in 0..coords.len() - 1 {
        let (x1, y1) = coords[i];
        for j in i + 1..coords.len() {
            let (x2, y2) = coords[j];
            let area = (y2.abs_diff(y1) + 1) * (x2.abs_diff(x1) + 1);
            max_area = if area > max_area { area } else { max_area };
        }
    }

    max_area
}

pub fn p2(input: &str) -> u64 {
    let coords: Vec<(u64, u64)> = input
        .lines()
        .map(|x| {
            let mut split = x.splitn(2, ',');
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    let mut max_area = 0;
    
    max_area
}

#[cfg(test)]
mod tests {
    use crate::d9::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d9_test.txt");
        assert_eq!(p1(input), 50)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d9_test.txt");
        assert_eq!(p2(input), 24)
    }
}
