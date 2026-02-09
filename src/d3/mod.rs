pub fn p1(input: &str) -> u64 {
    let mut counter = 0;

    for line in input.lines() {
        let mut max_one = 1;
        let mut max_two = 1;
        let line_str = line.to_string();
        let line_len = line_str.chars().count();

        for (i, current) in line_str.chars().enumerate() {
            let current: u64 = current.to_string().parse().expect("NaN");

            if i == line_len - 1 {
                if current > max_one {
                    max_one = current
                }
                break
            }

            // Parse max result, keep both the maximum and second maximum
            if current > max_two
            {
                max_two = current;
                max_one = 1;
            }
            else if current > max_one
            {
                max_one = current;
            }
        }

        let mut sum = 0;
        sum = 10 * max_two + max_one;
        counter += sum;
        // println!("sum: {sum} et counter: {counter}");
    }

    counter
}

pub fn p2(input: &str) -> u64 {
    return 42;
    // todo!()
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
