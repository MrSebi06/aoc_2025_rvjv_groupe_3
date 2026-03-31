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

pub fn voltage_to_lights(voltage: &Vec<usize>) -> Vec<bool> {
    let mut res = vec![false; 0];
    for v in voltage {
        res.push(v % 2 == 1)
    }
    res
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
        let mut rem_buttons_global: Vec<usize> = (0..buttons.len()).collect();
        for i in 0..buttons.len() {
            rem_buttons_global.remove(0);

            let buffer = BfsBuffer {
                lights_state: vec![false; expected_lights.len()],
                button_index: i,
                remaining_buttons: rem_buttons_global.clone(),
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

// First, a semi-copy-paste of P1. We'll need it for P2
// The major difference is, this time, we want to return every possible button combination that worked as a list of buttons to use later
#[derive(Default)]
struct BfsBufferP2 {
    lights_state: Vec<bool>,
    button_index: usize,
    remaining_buttons: Vec<usize>,
    pressed_buttons: Vec<usize>,
    depth: usize,
}

pub fn is_voltage_even(voltage: &Vec<usize>) -> bool {
    for v in voltage {
        if *v % 2 != 0 {
            return false;
        }
    }
    true
}

pub fn is_voltage_null(voltage: &Vec<usize>) -> bool {
    for v in voltage {
        if *v != 0 {
            return false;
        }
    }
    true
}

pub fn get_lights_hash(lights: &Vec<bool>) -> usize {
    let mut res = 0;
    for i in 0..lights.len() {
        if lights[i] {
            res += usize::pow(2, i as u32)
        }
    }
    res
}

pub fn brute_force_cache(buttons: &Vec<Vec<usize>>, len: usize, max_depth: usize) -> Vec<Option<Vec<Vec<usize>>>> {
    let mut cache = vec![None; usize::pow(2, len as u32)];

    let mut queue: VecDeque<BfsBufferP2> = VecDeque::new();
    let mut rem_buttons_global: Vec<usize> = (0..buttons.len()).collect();
    for i in 0..buttons.len() {
        let buffer = BfsBufferP2 {
            lights_state: vec![false; len],
            button_index: i,
            remaining_buttons: rem_buttons_global.clone(),
            pressed_buttons: vec![0; 0],
            depth: 1,
        };
        queue.push_back(buffer);

        rem_buttons_global.remove(0);
    }

    while !queue.is_empty() {
        let buffer = queue.pop_front().unwrap();
        let button = &buttons[buffer.button_index];

        let mut lights_state = buffer.lights_state;
        // Apply button operations to our current state
        for i in button {
            lights_state[*i] = !lights_state[*i]
        }

        let mut pre_buttons = buffer.pressed_buttons;
        pre_buttons.push(buffer.button_index);

        let hash = get_lights_hash(&lights_state);
        if cache[hash].is_none() {
            cache[hash] = Some(vec![pre_buttons.clone(); 1])
        } else {
            cache[hash].as_mut().unwrap().push(pre_buttons.clone());
        }

        if buffer.depth >= max_depth { continue }

        // If not, then start pushing further possibilities into the queue
        let mut rem_buttons = buffer.remaining_buttons.clone();
        for i in buffer.remaining_buttons.iter() {
            rem_buttons.remove(0);

            let new_buffer = BfsBufferP2 {
                lights_state: lights_state.clone(),
                button_index: *i,
                remaining_buttons: rem_buttons.clone(),
                pressed_buttons: pre_buttons.clone(),
                depth: buffer.depth + 1,
            };
            queue.push_back(new_buffer)
        }

        /*
        if buffer.remaining_buttons.is_empty()
        {
            for i in 0..buttons.len() {
                let new_buffer = BfsBufferP2 {
                    lights_state: lights_state.clone(),
                    button_index: i,
                    remaining_buttons: vec![],
                    pressed_buttons: pre_buttons.clone(),
                    depth: max_depth,
                };
                queue.push_back(new_buffer)
            }
        }
         */
    }

    cache
}

pub fn brute_force_cache_rev(buttons: &Vec<Vec<usize>>, len: usize, max_depth: usize) -> Vec<Option<Vec<Vec<usize>>>> {
    let mut cache = vec![None; usize::pow(2, len as u32)];

    let mut queue: VecDeque<BfsBufferP2> = VecDeque::new();
    let mut rem_buttons_global: Vec<usize> = (0..buttons.len()).rev().collect();
    for i in (0..buttons.len()).rev() {
        let buffer = BfsBufferP2 {
            lights_state: vec![false; len],
            button_index: i,
            remaining_buttons: rem_buttons_global.clone(),
            pressed_buttons: vec![0; 0],
            depth: 1,
        };
        queue.push_back(buffer);

        rem_buttons_global.remove(0);
    }

    while !queue.is_empty() {
        let buffer = queue.pop_front().unwrap();
        let button = &buttons[buffer.button_index];

        let mut lights_state = buffer.lights_state;
        // Apply button operations to our current state
        for i in button {
            lights_state[*i] = !lights_state[*i]
        }

        let mut pre_buttons = buffer.pressed_buttons;
        pre_buttons.push(buffer.button_index);

        let hash = get_lights_hash(&lights_state);
        if cache[hash].is_none() {
            cache[hash] = Some(vec![pre_buttons.clone(); 1])
        } else {
            cache[hash].as_mut().unwrap().push(pre_buttons.clone());
        }

        if buffer.depth >= max_depth { continue }

        // If not, then start pushing further possibilities into the queue
        let mut rem_buttons = buffer.remaining_buttons.clone();
        for i in buffer.remaining_buttons.iter() {
            rem_buttons.remove(0);

            let new_buffer = BfsBufferP2 {
                lights_state: lights_state.clone(),
                button_index: *i,
                remaining_buttons: rem_buttons.clone(),
                pressed_buttons: pre_buttons.clone(),
                depth: buffer.depth + 1,
            };
            queue.push_back(new_buffer)
        }
    }

    cache
}

// This recursive approach is taken from u/tenthmascot's fascinating post on Reddit: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
// The Rust implementation, of course, is home-made with love
pub fn presses_for_voltage(
    voltage: &Vec<usize>,
    buttons: &Vec<Vec<usize>>,
    solutions_cache: &mut Vec<Option<Vec<Vec<usize>>>>,
) -> u64 {
    // Recursive exit condition: If all voltages == 0, then we are done.
    if is_voltage_null(voltage) {
        return 0;
    }

    // What should the indicator lights look like when we're done...?
    let lights = voltage_to_lights(&voltage);

    // To avoid executing a BFS everytime, we'll cache the solutions found for each state.
    let hash = get_lights_hash(&lights);
    let mut solutions;

    // What are the solutions to reach these indicator lights according to P1?
    let solutions_check = solutions_cache[hash].clone();
    if solutions_check.is_none() {
        solutions = vec![];
    } else {
        solutions = solutions_check.unwrap()
    }

    let mut result: Option<u64> = None;
    for solution in solutions {
        // Let's apply this solution to our voltage, and then divide the resulting voltage by 2
        let mut new_voltage = voltage.clone();
        let mut invalid_flag = false;
        for button_i in solution.clone() {
            for i in buttons[button_i].clone() {
                // Be careful of invalid solutions that overflow
                if new_voltage[i] == 0 {
                    invalid_flag = true;
                    break;
                } else {
                    new_voltage[i] -= 1
                };
            }
            if invalid_flag {
                break;
            }
        }

        if invalid_flag {
            continue;
        }

        let mut divisions = 0;
        if !is_voltage_null(&new_voltage) {
            while is_voltage_even(&new_voltage) {
                for i in 0..new_voltage.len() {
                    new_voltage[i] /= 2
                }
                divisions += 1;
            }
        }

        let presses = presses_for_voltage(&new_voltage, &buttons, solutions_cache);
        let count = u64::pow(2, divisions) * presses + solution.len() as u64;

        if count < result.unwrap_or(404000) {
            // You can't get much lower than 1 so...
            if count == 1 {
                return 1;
            }
            result = Some(count);
        }
    }

    result.unwrap_or(404000)
}

pub fn p2(input: &str) -> u64 {
    let mut count = 0;

    for line in input.lines() {
        // Input format is messy, long process to treat it correctly
        let mut split = line.split(" ");

        // Part 2, we can skip lights
        split.next();

        let mut buttons = vec![vec![0; 0]; 0];
        let mut voltage = vec![0; 0];
        for (i, button_str) in split.enumerate() {
            if button_str.starts_with('{') {
                let button_str = &button_str[1..button_str.len() - 1];
                let voltage_indicators = button_str.split(",");
                for voltage_str in voltage_indicators {
                    voltage.push(voltage_str.parse::<usize>().unwrap())
                }

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

        let mut solutions_cache = brute_force_cache(&buttons, voltage.len(), buttons.len());
        let mut solutions_cache_rev = brute_force_cache_rev(&buttons, voltage.len(), buttons.len());

        let presses = presses_for_voltage(&voltage, &buttons, &mut solutions_cache);
        let presses_rev = presses_for_voltage(&voltage, &buttons, &mut solutions_cache_rev);
        if presses != presses_rev {
            println!("{line}")
        }

        count += presses;
        println!("Progress...")
    }

    count
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
        assert_eq!(p2(input), 33)
    }
}
