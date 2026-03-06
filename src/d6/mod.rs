pub fn p1(input: &str) -> u64 {
    let mut counter = 0;
    let mut numbers = vec![0u64; 0];

    let mut len_problem = 0;
    // Get table
    for line in input.lines() {
        let line_str = line.to_string();

        // I love whoever implemented this split
        let mut split = line_str.split_whitespace();

        for (i, to_parse) in split.enumerate()
        {
            // If it's a number, add it to the list
            let parsed_number = to_parse.parse::<u64>();
            if !parsed_number.is_err()
            {
                numbers.push(parsed_number.unwrap());
                continue;
            }

            // Resolving is simple:
            //   We know the list is, in fact, a SQUARE of equations.
            //   This means that the next number in a problem is directly below the last
            //   i.e: numbers[0] * numbers[0 + len_problem] * numbers[0 + 2*len_problem] etc
            let mut res = numbers[i];
            match to_parse
            {
                "+" => { for j in 1..len_problem - 1 { res += numbers[j * len_problem + i] } }
                "*" => { for j in 1..len_problem - 1 { res *= numbers[j * len_problem + i] } }
                _ => {}
            }

            counter += res;
        }

        // Memorize problem length on first pass
        if (len_problem == 0)
        {
            len_problem = numbers.iter().count();
        }

        continue;
    }

    counter
}

pub fn p2(input: &str) -> u64 {
    let mut ranges = vec![(0u64, 0u64); 0];

    // Cache all ranges
    for line in input.lines() {
        let line_str = line.to_string();

        if line_str != "" {
            let mut split = line_str.split("-");

            ranges.push((
                split.next().expect("Oups").parse::<u64>().unwrap(),
                split.next().expect("Oups").parse::<u64>().unwrap(),
            ));
        } else {
            break;
        }
    }

    // Optimize ranges
    let mut len = ranges.len();
    let mut i = 0;
    while i < len {
        let (mut current_lower, mut current_upper) = ranges[i];
        // println!("current: {current_lower}-{current_upper}");

        let mut j = 0;
        while j < len {
            if j == i {
                j += 1;
                continue;
            }

            let (other_lower, other_upper) = ranges[j];
            // println!("other: {other_lower}-{other_upper}");

            if current_upper <= other_upper && current_upper >= other_lower {
                current_upper = other_upper;
                if other_lower <= current_lower {
                    current_lower = other_lower
                }
                // println!("new range: {current_lower}-{current_upper}");
                ranges[i] = (current_lower, current_upper);
                ranges.remove(j);
                len -= 1;
                j = 0;
                i = 0;

                (current_lower, current_upper) = ranges[0];
            } else if current_lower <= other_upper && current_lower >= other_lower {
                current_lower = other_lower;
                if other_upper >= current_upper {
                    current_upper = other_upper
                }
                // println!("new range: {current_lower}-{current_upper}");
                ranges[i] = (current_lower, current_upper);
                ranges.remove(j);
                len -= 1;
                j = 0;
                i = 0;

                (current_lower, current_upper) = ranges[0];
            }

            j += 1;
        }

        i += 1;
    }

    // Count results
    let mut counter = 0;
    for (lower, upper) in ranges.iter() {
        // println!("range: {lower}-{upper}");
        counter += upper - lower + 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::d6::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d6_test.txt");
        assert_eq!(p1(input), 4277556)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d6_test.txt");
        assert_eq!(p2(input), 202)
    }
}
