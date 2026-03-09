use std::collections::HashSet;

pub fn p1(input: &str) -> u64 {

    let mut count = 0;
    let mut active_spaces = HashSet::<usize>::new();

    for line in input.lines() {
        let mut next_active_spaces = HashSet::<usize>::new();

        for (i, cur_space) in line.chars().enumerate()
        {
            match cur_space
            {
                'S' => { next_active_spaces.insert(i); },
                '.' => { if active_spaces.contains(&i) { next_active_spaces.insert(i); } },
                '^' => {
                    if active_spaces.contains(&i)
                    {
                        next_active_spaces.insert(i - 1);
                        next_active_spaces.insert(i + 1);
                        count += 1;
                    }
                },
                _ => {}
            }
        }

        active_spaces = next_active_spaces;
    }

    count
}

pub fn p2(input: &str) -> u64 {
    // SPOILER: Parcours d'arbre binaire!
    
    0
}

#[cfg(test)]
mod tests {
    use crate::d7::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d7_test.txt");
        assert_eq!(p1(input), 21)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d7_test.txt");
        assert_eq!(p2(input), 40)
    }
}
