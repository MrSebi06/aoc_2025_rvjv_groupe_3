fn is_invalid_id_p1(id: String) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        return false;
    }
    let (first, last) = id.split_at(len / 2);
    first == last
}

fn is_invalid_id_p2_n(id: u64, len: u32, n: u32) -> bool {

    if len % n != 0 { return false }

    let pow10 = 10u64.pow(len / n);
    let pattern = id % pow10;

    let mut temp_id = id;
    for _ in 1..n
    {
        temp_id /= pow10;
        if temp_id % pow10 != pattern { return false }
    }

    true
}

fn is_invalid_id_p2(id: u64) -> bool {
    let len = id.ilog10() + 1;

    for i in 2..=len
    {
        if is_invalid_id_p2_n(id, len, i) { return true }
    }

    false
}

pub fn p1(input: &str) -> u64 {
    let segments = input.trim().split(",");
    let mut total = 0;

    for segment in segments {
        let mut bounds = segment.split('-');
        let lower: u64 = bounds
            .next()
            .expect("lower not found")
            .to_string()
            .parse()
            .expect("lower is not u64");
        let upper: u64 = bounds
            .next()
            .expect("upper not found")
            .to_string()
            .parse()
            .expect("upper is not u64");

        for i in lower..=upper {
            let len = i.ilog10() + 1;
            if is_invalid_id_p2_n(i, len, 2) {
                total += i;
            }
        }
    }

    total
}

pub fn p2(input: &str) -> u64 {
    let segments = input.trim().split(",");
    let mut total = 0;

    for segment in segments {
        let mut bounds = segment.split('-');
        let lower: u64 = bounds
            .next()
            .expect("lower not found")
            .to_string()
            .parse()
            .expect("lower is not u64");
        let upper: u64 = bounds
            .next()
            .expect("upper not found")
            .to_string()
            .parse()
            .expect("upper is not u64");

        for i in lower..=upper {
            if is_invalid_id_p2(i) {
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
