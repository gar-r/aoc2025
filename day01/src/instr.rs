use util::Dir::{self, Left, Right};

pub fn parse(instr: &str) -> (Dir, i32) {
    let val = instr[1..].parse::<i32>().unwrap();
    match instr.chars().nth(0).unwrap() {
        'L' => (Left, val),
        'R' => (Right, val),
        _ => panic!("error parsing: {}", instr),
    }
}

#[cfg(test)]
mod tests {
    use util::Dir::Right;

    use crate::instr::parse;

    #[test]
    fn test_instr_parse() {
        let (d, v) = parse("R18");
        assert_eq!(Right, d);
        assert_eq!(18, v);
    }

    #[test]
    #[should_panic]
    fn test_instr_parse_error() {
        let (_, _) = parse("AB19");
    }
}
