pub struct Worksheet {
    pub problems: Vec<Problem>,
}

pub struct Problem {
    pub numbers: Vec<u64>,
    pub operator: char,
}

impl Worksheet {
    pub fn checksum(&self) -> u64 {
        self.problems.iter().fold(0, |sum, p| sum + p.solve())
    }
}

impl Problem {
    pub fn new() -> Problem {
        Problem { numbers: Vec::new(), operator: '+' }
    }

    pub fn solve(&self) -> u64 {
        match self.operator {
            '*' => self.numbers.iter().fold(1, |sum, n| sum * n),
            '+' => self.numbers.iter().sum(),
            _ => panic!("invalid operator"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::worksheet::{Problem, Worksheet};

    #[test]
    fn test_problem_solve() {
        assert_eq!(
            33210,
            Problem {
                numbers: vec![123, 45, 6],
                operator: '*'
            }
            .solve()
        );

        assert_eq!(
            490,
            Problem {
                numbers: vec![328, 64, 98],
                operator: '+'
            }
            .solve()
        );

        assert_eq!(
            4243455,
            Problem {
                numbers: vec![51, 387, 215],
                operator: '*'
            }
            .solve()
        );

        assert_eq!(
            401,
            Problem {
                numbers: vec![64, 23, 314],
                operator: '+'
            }
            .solve()
        );
    }

    #[test]
    fn test_worksheet_checksum() {
        let lines = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        let w = Worksheet::parse(lines);
        assert_eq!(4277556, w.checksum());
    }
}
