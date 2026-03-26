use std::collections::HashMap;

pub fn p1(input: &str) -> u64 {
    let nodes: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let mut split = line.split(':');
        let node = split.next().expect("invalid");
        let childs = split
            .next()
            .expect("invalid")
            .split(" ")
            .collect::<Vec<&str>>();
        println!("childs}");
    }
    0
}

pub fn p2(input: &str) -> u64 {
    0
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
        let input = include_str!("d11_test.txt");
        assert_eq!(p2(input), 24)
    }
}
