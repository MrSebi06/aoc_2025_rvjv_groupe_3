use std::collections::VecDeque;

#[derive(Default)]
struct BfsBuffer {
    lights_state: Vec<bool>,
    button_index: usize,
    remaining_buttons: Vec<usize>,
    depth: u64,
}

pub fn check_res(expected: &Vec<bool>, current: &Vec<bool>) -> bool {
    for i in 0..expected.len() {
        if expected[i] != current[i] {
            return false;
        }
    }

    true
}

pub fn p1(input: &str) -> u64 {
    let mut count = 0;

    for line in input.lines() {
        // Input format is messy, long process to treat it correctly
        let mut split = line.split(" ");

        // First is always the expected lights display, so we convert it to a vec of booleans
        let expected_lights_str = split.next().unwrap();
        let expected_lights_str = &expected_lights_str[1..expected_lights_str.len() - 1];

        let mut expected_lights = vec![false; 0];
        for c in expected_lights_str.chars() {
            match c {
                '.' => expected_lights.push(false),
                '#' => expected_lights.push(true),
                _ => {}
            }
        }

        let mut buttons = vec![vec![0; 0]; 0];
        for (i, button_str) in split.enumerate() {
            // Part 1, we can skip voltages
            if button_str.starts_with('{') {
                continue;
            }

            let mut triggers = vec![0; 0];

            let button_str = &button_str[1..button_str.len() - 1];
            let button_triggers = button_str.split(",");
            for trig_str in button_triggers {
                triggers.push(trig_str.parse::<usize>().unwrap())
            }

            buttons.push(triggers);
        }

        // god this is a mess and the algorithm hasn't even started
        // Speaking of, we'll be iterating over the possibilities using a BFS, as if this were a graph.
        // There's probably more optimal but I'm short on sanity and time so we'll just...

        let mut queue: VecDeque<BfsBuffer> = VecDeque::new();
        let rem_buttons_global: Vec<usize> = (0..buttons.len()).collect();
        for i in 0..buttons.len() {
            let mut rem_buttons = rem_buttons_global.clone();
            rem_buttons.remove(i);

            let buffer = BfsBuffer {
                lights_state: vec![false; expected_lights.len()],
                button_index: i,
                remaining_buttons: rem_buttons,
                depth: 1,
            };
            queue.push_back(buffer)
        }

        let mut explored_states: Vec<Vec<bool>> = vec![vec![false; expected_lights.len()]; 1];
        while !queue.is_empty() {
            let buffer = queue.pop_front().unwrap();
            let button = &buttons[buffer.button_index];

            let mut lights_state = buffer.lights_state;
            // Apply button operations to our current state
            for i in button {
                lights_state[*i] = !lights_state[*i]
            }

            let mut explored = false;
            for explored_state in explored_states.iter() {
                if check_res(&lights_state, &explored_state) {
                    explored = true;
                    break;
                }
            }
            if explored {
                continue;
            }

            explored_states.push(lights_state.clone());

            // Test if current result fits our expected result...
            if check_res(&expected_lights, &lights_state) {
                count += buffer.depth;
                break;
            }

            // If not, then start pushing further possibilities into the queue
            // TODO: Trim the sent possibilities so we don't lose time on useless ones (For example using the same button twice)
            // That is easily the best way to optimize this approach
            for i in buffer.remaining_buttons.iter() {
                let mut rem_buttons = buffer.remaining_buttons.clone();
                rem_buttons.remove(rem_buttons.iter().position(|&r| &r == i).unwrap());

                let new_buffer = BfsBuffer {
                    lights_state: lights_state.clone(),
                    button_index: *i,
                    remaining_buttons: rem_buttons,
                    depth: buffer.depth + 1,
                };
                queue.push_back(new_buffer)
            }
        }
    }

    count
}

pub fn p2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::d10::{p1, p2};

    #[test]
    fn p1_test() {
        let input = include_str!("d10_test.txt");
        assert_eq!(p1(input), 7)
    }

    #[test]
    fn p2_test() {
        let input = include_str!("d10_test.txt");
        assert_eq!(p2(input), 7)
    }
}
