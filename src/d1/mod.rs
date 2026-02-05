use std::ops::Rem;

pub fn p1(input: &str) -> usize {
    let mut cur_pos = 50;
    let mut counter = 0;

    for line in input.lines() {
        let mut line_str = line.to_string();
        let direction = line_str.remove(0);
        let distance: i16 = line_str.parse().expect("NaN");

        let direction: i16 = match direction {
            'L' => -1,
            _ => 1
        };

        cur_pos = (cur_pos + (direction * distance)) % 100;
        if cur_pos == 0 {
            counter += 1
        }
    }
    counter
}

pub fn p2(input: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use crate::d1::p1;

    #[test]
    fn p1_test() {
        let input = include_str!("d1_test.txt");
        assert_eq!(p1(input), 3)
    }
}
