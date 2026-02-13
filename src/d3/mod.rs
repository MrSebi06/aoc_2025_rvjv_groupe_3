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

pub fn p2_mika(input: &str) -> u64 {
    const NB_JOLTAGES: usize = 12;

    let mut counter: u64 = 0;

    for line in input.lines() {
        let line_str = line.to_string();
        let line_len = line_str.chars().count();

        let mut joltages = [1; NB_JOLTAGES];
        let mut joltage_id = 0;

        let max_skips = line_len - NB_JOLTAGES;
        let mut skipped_count = 0;

        for current in line_str.chars() {
            let current: u64 = current.to_string().parse().expect("NaN");

            while joltage_id > 0 && skipped_count < max_skips && joltages[joltage_id - 1] < current {
                joltage_id -= 1;
                skipped_count += 1;
            }

            if joltage_id < NB_JOLTAGES {
                joltages[joltage_id] = current;
                joltage_id += 1;
            } else if skipped_count < max_skips {
                skipped_count += 1;
            }
        }

        let mut sum = 0;
        for joltage_id in 0..joltages.len() {
            sum += (10u64.pow((NB_JOLTAGES - joltage_id - 1) as u32) * joltages[joltage_id]);
            // print!("sum: {sum}, ");
        }
        counter += sum;
        // println!("counter: {counter} et skipped_count: {skipped_count} et max_skips: {max_skips} et joltage_id: {joltage_id} et NB_JOLTAGES: {NB_JOLTAGES} et joltages: {:?}", joltages);
    }

    counter
}

pub fn p2_sacha(input: &str, digits: u64) -> u64
{
    // For the sake of usability, adapt digits to our use
    let digits = digits - 1;
    let mut res = 0;

    for line in input.lines()
    {
        let chars_vec: Vec<char> = line.to_string().chars().collect();
        let len = chars_vec.len();

        // Assume n as the order of magnitude
        let mut list_start = 0;
        let mut cur_res = 0;
        for n in 0..=digits
        {
            let n = digits - n;
            let sub_chars = &chars_vec[list_start..(len - n as usize)];

            // Search max in sub-list
            let cur_list_start = list_start;
            let mut max = 1;
            for (i, x) in sub_chars.iter().enumerate().map(|(i, x)| { (i, x.to_digit(10).unwrap()) })
            {
                let x = x as u64;
                if max < x
                {
                    max = x;
                    list_start =  cur_list_start + i;
                }
            }

            cur_res = cur_res * 10u64 + max;
            list_start += 1;
        }

        res += cur_res;
    }

    res
}

pub fn p2_seb(input: &str) -> u64 {
    const STACK_SIZE: usize = 12;
    let mut sum = 0;

    for line in input.lines() {
        let line_str = line.to_string();
        let line_len = line.chars().count();
        let mut stack: Vec<u8> = vec![];

        for (i, current) in line_str.chars().enumerate() {
            let current: u8 = current.to_string().parse().expect("NaN");

            let remaining = line_len - i - 1;
            let mut pops = 0;

            if  stack.last().is_some_and(|&last| current ==  last)  && remaining + stack.len() >= STACK_SIZE {
                continue
            }

            while stack.last().is_some_and(|&last| current >  last) && remaining + stack.len() >= STACK_SIZE {
                if pops >= remaining { break; }
                stack.pop();
                pops += 1;
            }
            stack.push(current);
        }
        // println!("{:?}", stack);
        let result: u64 = stack.iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        sum += result;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::d3::{p1, p2_mika, p2_sacha, p2_seb};

    #[test]
    fn p1_test() {
        let input = include_str!("d3_test.txt");
        assert_eq!(p1(input), 357)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d3_test.txt");
        assert_eq!(p2_mika(input), 3121910778619)
    }
}
