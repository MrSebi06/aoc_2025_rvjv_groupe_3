fn is_invalid_id_p1(id: String) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        return false;
    }
    let (first, last) = id.split_at(len / 2);
    first == last
}

fn is_invalid_id_p2(id: String) -> bool {
    let len = id.len();

    for i in 1..=len / 2 {
        if len % i != 0 {
            continue;
        }
        let sections = id
            .chars()
            .collect::<Vec<char>>()
            .chunks(i)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>();

        if sections.iter().all(|section| {
            *section == sections[0]
        }) {
            return true
        }
    }
    false
}

pub fn p1(input: &str) -> u64 {
    let segments = input.split(",");
    let mut total = 0;

    for segment in segments {
        let mut bounds = segment.split('-');
        let lower: u64 = bounds
            .next()
            .expect("lower not found")
            .to_string()
            .parse()
            .expect("lower is not u32");
        let upper: u64 = bounds
            .next()
            .expect("upper not found")
            .to_string()
            .parse()
            .expect("upper is not u32");

        for i in lower..=upper {
            let i_str = i.to_string();

            if is_invalid_id_p1(i_str) {
                total += i;
            }
        }
    }

    total
}

pub fn p2(input: &str) -> u64 {
    let segments = input.split(",");
    let mut total = 0;

    for segment in segments {
        let mut bounds = segment.split('-');
        let lower: u64 = bounds
            .next()
            .expect("lower not found")
            .to_string()
            .parse()
            .expect("lower is not u32");
        let upper: u64 = bounds
            .next()
            .expect("upper not found")
            .to_string()
            .parse()
            .expect("upper is not u32");

        for i in lower..=upper {
            let i_str = i.to_string();

            if is_invalid_id_p2(i_str) {
                total += i;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::d2::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d2_test.txt");
        assert_eq!(p1(input), 1227775554)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d2_test.txt");
        assert_eq!(p2(input), 4174379265)
    }
}
