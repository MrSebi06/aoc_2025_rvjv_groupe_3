pub fn p1(input: &str) -> u64 {
    let mut counter = 0;
    let mut numbers = vec![0u64; 0];

    let mut len_line = 0;
    let mut len_problem = 0;
    // Get table
    for line in input.lines() {
        let line_str = line.to_string();

        // I love whoever implemented this split
        let split = line_str.split_whitespace();

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
            //   We know the list is, in fact, a RECTANGLE of equations.
            //   This means that the next number in a problem is directly below the last
            //   i.e: numbers[0] * numbers[0 + len_problem] * numbers[0 + 2*len_problem] etc
            let mut res = numbers[i];
            match to_parse
            {
                "+" => { for j in 1..len_problem { res += numbers[j * len_line + i] } }
                "*" => { for j in 1..len_problem { res *= numbers[j * len_line + i] } }
                _ => {}
            }

            counter += res;
        }

        len_problem += 1;
        // Memorize problem length on first pass
        if (len_line == 0)
        {
            len_line = numbers.iter().count();
        }

        continue;
    }

    counter
}

pub fn p2(input: &str) -> u64 {
    let mut result = 0u64;

    let mut lines = input.lines().collect::<Vec<&str>>();
    let lines_len = lines.len();

    let first_line_len = lines.first().map_or(0, |l| l.len());
    let mut padded = None;
    if let Some(last) = lines.last() {
        if last.len() < first_line_len {
            padded = Some(format!("{:<width$}", last, width = first_line_len));
        }
    }
    if let Some(p) = &padded {
        lines.pop();
        lines.push(p.as_str());
    }

    let mut i = 0;
    let mut char: char;

    let mut numbers: Vec<u64> = vec![];
    let mut number: String = String::new();

    loop {
        let line = lines[i];
        let mut chars = line.chars();

        let expected = chars.next_back();
        if expected.is_none(){
            println!("Line HERE: {line}")
        }
        char = expected.unwrap();
        lines[i] = chars.as_str();

        if i == lines_len - 1 {
            let mut res = 0;

            let trimmed_number = number.trim();
            if trimmed_number.len() != 0 {
                numbers.push(trimmed_number.parse::<u64>().unwrap());
            }

            match char {
                 '+' => {
                     for num in numbers.iter() {res += num}
                     numbers.clear();
                     result += res;
                 },
                 '*' => {
                     res = 1;
                     for num in numbers.iter() {res *= num}
                     numbers.clear();
                     result += res;
                 },
                 _ => {}
            }

            i = 0;
            number = String::new();

            if lines[i].len() == 0 {
                break;
            }

            continue;
        }

        number.push(char);
        i = i+1;
    }

    result
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
        assert_eq!(p2(input), 3263827)
    }
}
