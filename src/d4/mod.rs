pub fn p1(input: &str) -> u64 {
    let mut counter = 0;

    let mut table = vec![vec![false; 0]; 0];

    // Get table
    for line in input.lines() {
        let line_str = line.to_string();
        let line_len = line_str.chars().count();

        let mut line = vec![false; line_len];

        for (i, current) in line_str.chars().enumerate() {
            line[i] = current == '@'
        }

        table.push(line)
    }

    // Algo
    let lines = table.len();
    let columns = table[0].len();
    
    for line in 0..lines
    {
        for column in 0..columns
        {
            if !table[line][column] { continue }

            let mut neighbors = 0;
            for x in -1i32..=1 {
                // Edge cases
                // Too much on the left
                if x == -1 && column == 0 { continue }
                // Too much on the right
                if x == 1 && column == columns - 1 { continue }

                for y in -1i32..=1 {
                    // Edge cases
                    // Too much on the left
                    if y == -1 && line == 0 { continue }
                    // Too much on the right
                    if y == 1 && line == lines - 1 { continue }

                    let x = (column as i32 + x) as usize;
                    let y = (line as i32 + y) as usize;

                    if line == y && column == x { continue }

                    if table[y][x] { neighbors += 1 }
                }
            }

            // If neighbors are manageable, add counter
            if neighbors < 4 {
                counter += 1;
            }
        }
    }

    counter
}

pub fn p2(input: &str) -> u64 {
    let mut counter = 0;

    for line in input.lines() {

        let line_str = line.to_string();
        let line_len = line_str.chars().count();

        for (i, current) in line_str.chars().enumerate() {

        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::d4::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d4_test.txt");
        assert_eq!(p1(input), 13)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d4_test.txt");
        assert_eq!(p2(input), 13)
    }
}
