use util::Dir;

pub struct Dial {
    pub val: i32,
    pub of: i32,
}

impl Dial {
    pub fn rotate(&mut self, dir: Dir, amount: i32) {
        let n = if dir == Dir::Left { -1 } else { 1 };
        self.of = 0;
        for _ in 0..amount {
            self.val += n;
            self.handle_overflow();
        }
    }

    fn handle_overflow(&mut self) {
        self.val = self.val % 100;
        if self.val == 0 {
            self.of += 1;
        }
        if self.val < 0 {
            self.val = 100 + self.val;
        }
    }
}

#[cfg(test)]
mod test {
    use util::Dir::{Left, Right};

    use crate::dial::Dial;

    #[test]
    fn test_dial_rotate() {
        let d = &mut Dial { val: 50, of: 0 };

        d.rotate(Left, 68);
        assert_eq!(82, d.val);
        assert_eq!(1, d.of);

        d.rotate(Left, 30);
        assert_eq!(52, d.val);
        assert_eq!(0, d.of);

        d.rotate(Right, 48);
        assert_eq!(0, d.val);
        assert_eq!(1, d.of);
    }

    #[test]
    fn test_dial_rotate_large() {
        let d = &mut Dial { val: 50, of: 0 };
        d.rotate(Right, 888);
        assert_eq!(38, d.val);
        assert_eq!(9, d.of);
    }
}
