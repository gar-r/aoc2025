use util::read_input_lines;

fn main() {
    let input = read_input_lines("input/day03-real.txt").unwrap();
    println!("part 1: {}", calc_max_joltage(&input, 2));
    println!("part 2: {}", calc_max_joltage(&input, 12));
}

pub fn calc_max_joltage(input: &Vec<String>, digits: usize) -> u64 {
    input
        .iter()
        .map(|batt| max_joltage(&batt, digits))
        .map(|s| s.parse::<u64>().unwrap())
        .fold(0, |acc, x| acc + x)
}

pub fn max_joltage(batteries: &str, digits: usize) -> String {
    let mut result = String::new();
    let mut idx: Option<usize> = None;
    for digit in (0..digits).rev() {
        let start_idx = if idx.is_none() { 0 } else { idx.unwrap() + 1 };
        let end_idx = batteries.len() - digit;
        let (c_idx, c) = max(batteries, start_idx, end_idx);
        result.push(c);
        idx = Some(c_idx);
    }
    result
}

fn max(batt: &str, start: usize, end: usize) -> (usize, char) {
    let mut max_idx = start;
    let mut max_val = batt.chars().nth(start).unwrap();
    for i in start + 1..end {
        let c = batt.chars().nth(i).unwrap();
        if c > max_val {
            max_idx = i;
            max_val = c;
        }
    }
    (max_idx, max_val)
}

#[cfg(test)]
mod tests {
    use crate::max_joltage;

    #[test]
    fn test_max_joltage_1() {
        assert_eq!("98", max_joltage("987654321111111", 2));
        assert_eq!("89", max_joltage("811111111111119", 2));
        assert_eq!("78", max_joltage("234234234234278", 2));
        assert_eq!("92", max_joltage("818181911112111", 2));
    }

    #[test]
    fn test_max_joltage_2() {
        assert_eq!("987654321111", max_joltage("987654321111111", 12));
        assert_eq!("811111111119", max_joltage("811111111111119", 12));
        assert_eq!("434234234278", max_joltage("234234234234278", 12));
        assert_eq!("888911112111", max_joltage("818181911112111", 12));
    }
}
