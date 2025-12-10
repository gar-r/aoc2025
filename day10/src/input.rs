use util::read_input_lines;

use crate::{Button, Machine};

pub fn parse_machines(path: &str) -> Vec<Machine> {
    let lines = read_input_lines(path).unwrap();
    let mut result = Vec::new();
    for line in lines {
        result.push(Machine {
            indicators: parse_indicators(&line),
            buttons: parse_buttons(&line),
            joltages: parse_joltages(&line),
        });
    }
    result
}

fn parse_indicators(s: &str) -> Vec<bool> {
    let start_idx = s.find('[').unwrap();
    let end_idx = s.find(']').unwrap();
    let indicator_str = &s[start_idx + 1..end_idx];
    let mut indicators = Vec::new();
    for c in indicator_str.chars() {
        match c {
            '.' => indicators.push(false),
            '#' => indicators.push(true),
            _ => (),
        }
    }
    indicators
}

fn parse_joltages(s: &str) -> Vec<u32> {
    let start_idx = s.find('{').unwrap();
    let end_idx = s.find('}').unwrap();
    let joltage_str = &s[start_idx + 1..end_idx];
    let mut joltages = Vec::new();
    for j in joltage_str.split(',') {
        joltages.push(j.parse().unwrap());
    }
    joltages
}

fn parse_buttons(s: &str) -> Vec<Button> {
    let start_idx = s.find(']').unwrap();
    let end_idx = s.find('{').unwrap();
    let buttons_str = &s[start_idx + 2..end_idx - 2];
    let mut buttons = Vec::new();
    for b in buttons_str.split(' ') {
        let button = parse_button(b);
        buttons.push(button);
    }
    buttons
}

fn parse_button(s: &str) -> Button {
    Button {
        wiring: s
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect(),
    }
}
