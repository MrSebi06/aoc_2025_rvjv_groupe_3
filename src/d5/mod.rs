pub fn p1(input: &str) -> u64 {
    let mut counter = 0;

    let mut ranges = vec![(0u64, 0u64); 0];

    // Get table
    let mut range = true;
    for line in input.lines() {
        let line_str = line.to_string();
        let line_len = line_str.chars().count();

        if line_str != "" {
            let mut split = line_str.split("-");

            if range {
                ranges.push((
                    split.next().expect("Oups").parse::<u64>().unwrap(),
                    split.next().expect("Oups").parse::<u64>().unwrap(),
                ));
            } else {
                // ici on en est au ID
                let number = split.next().expect("OH NOOOOO").parse::<u64>().unwrap();

                for (lower, upper) in ranges.iter()
                {
                    if number >= *lower && number <= *upper {
                        counter += 1;
                        break
                    }
                }
            }
            continue
        } else {
            range = false
        }
        for (i, current) in line_str.chars().enumerate() {}
    }

    counter
}

pub fn p2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::d5::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d5_test.txt");
        assert_eq!(p1(input), 3)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d5_test.txt");
        assert_eq!(p2(input), 14)
    }
}
