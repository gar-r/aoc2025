pub fn is_valid(n: i64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return true;
    }

    let mid = s.len() / 2;
    let s1 = s[..mid].to_owned();
    let s2 = s[mid..].to_owned();
    return !s1.eq(&s2);
}

pub fn is_valid_v2(n: i64) -> bool {
    let s = n.to_string();
    for l in 1..s.len() {
        let pattern = s[..l].to_owned();
        let res = s.replace(&pattern, "");
        if res.len() == 0 {
            return false; // invalid id
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::id::{is_valid, is_valid_v2};

    #[test]
    fn test_is_valid() {
        assert!(is_valid(123));
        assert!(is_valid(1011));

        assert!(!is_valid(55));
        assert!(!is_valid(6464));
        assert!(!is_valid(123123));
    }

    #[test]
    fn test_is_valid_v2() {
        assert!(is_valid_v2(123));
        assert!(is_valid_v2(1011));

        assert!(!is_valid_v2(11));
        assert!(!is_valid_v2(999));
        assert!(!is_valid_v2(12341234));
        assert!(!is_valid_v2(123123123));
        assert!(!is_valid_v2(38593859));
    }
}
