use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use crate::{Button, Machine};

pub fn find_min_button_presses(m: &Machine) -> Option<usize> {
    let initial = vec![false; m.indicators.len()];
    if m.indicators == initial {
        return Some(0); // nothing to do
    }

    let mut queue = VecDeque::new();
    queue.push_back((initial.clone(), Vec::new()));

    let mut visited = HashSet::new();
    visited.insert(initial);

    while let Some((state, presses)) = queue.pop_front() {
        for (i, b) in m.buttons.iter().enumerate() {
            let mut new_presses = presses.clone();
            new_presses.push(i);
            let new_state = press_button(&state, b);
            if new_state == m.indicators {
                return Some(new_presses.len());
            }
            // only queue work item if the state didn't occur earlier
            if visited.insert(new_state.clone()) {
                queue.push_back((new_state, new_presses));
            }
        }
    }
    None // solution not found
}

fn press_button(indicators: &Vec<bool>, btn: &Button) -> Vec<bool> {
    let mut result = indicators.clone();
    for w in &btn.wiring {
        result[*w] = !result[*w];
    }
    result
}
