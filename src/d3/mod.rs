pub fn p1(input: &str) -> u64 {
    todo!()
}

pub fn p2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::d3::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d3_test.txt");
        assert_eq!(p1(input), 357)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d3_test.txt");
        assert_eq!(p2(input), 0)
    }
}
