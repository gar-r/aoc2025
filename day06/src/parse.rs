use crate::worksheet::{Problem, Worksheet};

impl Worksheet {
    pub fn parse(input: Vec<&str>) -> Worksheet {
        // collect the input fields as a simple 2d array
        let data: Vec<Vec<String>> = input
            .iter()
            .map(|l| l.split_whitespace().map(|s| String::from(s)).collect())
            .collect();

        // "invert" the 2d array into a Worksheet struct
        let mut result = Worksheet {
            problems: Vec::new(),
        };
        for col in 0..data[0].len() {
            // each column is a "problem", the last row is the operator
            let mut p = Problem {
                numbers: Vec::new(),
                operator: data.last().unwrap()[col].chars().nth(0).unwrap(),
            };
            // parse the numbers (skip last row, which is the operator)
            for row in 0..data.len() - 1 {
                let val = &data[row][col];
                p.numbers.push(val.parse().unwrap());
            }
            result.problems.push(p);
        }
        result
    }
}

#[cfg(test)]
pub mod tests {
    use crate::worksheet::{Problem, Worksheet};

    #[test]
    fn test_worksheet_parse() {
        let lines = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        let w = Worksheet::parse(lines);

        assert_eq!(4, w.problems.len());

        assert_problem(&w.problems[0], '*', vec![123, 45, 6]);
        assert_problem(&w.problems[1], '+', vec![328, 64, 98]);
        assert_problem(&w.problems[2], '*', vec![51, 387, 215]);
        assert_problem(&w.problems[3], '+', vec![64, 23, 314]);
    }

    #[track_caller]
    pub fn assert_problem(p: &Problem, operator: char, numbers: Vec<u64>) {
        assert_eq!(operator, p.operator);
        assert_eq!(numbers.len(), p.numbers.len());
        for (i, n) in numbers.iter().enumerate() {
            assert_eq!(*n, p.numbers[i]);
        }
    }
}
