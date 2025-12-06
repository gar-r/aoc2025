use util::Map2d;

use crate::worksheet::{Problem, Worksheet};

impl Worksheet {
    pub fn parse_v2(input: &Map2d) -> Worksheet {
        let (len_x, len_y) = input.len();

        let mut result = Worksheet {
            problems: Vec::new(),
        };

        let mut p = Problem::new();

        // parse the numbers with reversed column order,
        // with the most significant digit at the top
        'col: for col in (0..len_y).rev() {
            // parse the digits
            let mut digits = String::new();
            for row in 0..len_x {
                let cell = input.get(row, col);
                if cell.is_some() {
                    let c = *cell.unwrap();
                    if c.is_digit(10) {
                        digits.push(c);
                    } else if Worksheet::is_operator(c) {
                        p.numbers.push(digits.parse().unwrap());
                        p.operator = c;
                        result.problems.push(p);
                        // start a new problem
                        p = Problem::new();
                        continue 'col;
                    }
                }
            }
            // add the number to the problem
            if digits.trim().len() > 0 {
                p.numbers.push(digits.parse().unwrap());
            }
        }
        result
    }

    fn is_operator(c: char) -> bool {
        c == '+' || c == '*'
    }
}

#[cfg(test)]
mod tests {
    use util::Map2d;

    use crate::{parse::tests::assert_problem, worksheet::Worksheet};


    #[test]
    fn test_parse_v2() {
        let lines = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        let data: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
        let map2d = Map2d { data: data };

        let w = Worksheet::parse_v2(&map2d);
        assert_eq!(4, w.problems.len());

        assert_problem(&w.problems[0], '+', vec![4, 431, 623]);
        assert_problem(&w.problems[1], '*', vec![175, 581, 32]);
        assert_problem(&w.problems[2], '+', vec![8, 248, 369]);
        assert_problem(&w.problems[3], '*', vec![356, 24, 1]);
    }

}