pub struct Map2d {
    pub data: Vec<Vec<char>>,
}

impl Map2d {
    pub fn len(&self) -> (usize, usize) {
        (
            self.data.len(),
            if self.data.get(0).is_none() {
                0
            } else {
                self.data.get(0).unwrap().len()
            },
        )
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.data.get(x)?.get(y)
    }

    pub fn set(&mut self, x: usize, y: usize, val: char) {
        self.data[x][y] = val;
    }

    // get the values of adjacent cells in clock-wise order, starting from "north"
    pub fn get_adjacent(&self, x: usize, y: usize) -> Vec<&char> {
        // collect adjecent coordinates, considering negative usize overflow
        let mut adj: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            adj.push((x - 1, y));
            adj.push((x - 1, y + 1));
        }
        adj.push((x, y + 1));
        adj.push((x + 1, y + 1));
        adj.push((x + 1, y));
        if y > 0 {
            adj.push((x + 1, y - 1));
            adj.push((x, y - 1));
            if x > 0 {
                adj.push((x - 1, y - 1));
            }
        }
        adj.iter()
            .map(|(x, y)| self.get(*x, *y))
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::map2d::Map2d;
    
    #[test]
    fn test_get_set() {
        let map2d = &mut Map2d {
            data: vec![vec!['a', 'b'], vec!['c', 'd']],
        };
        assert_eq!('a', *map2d.get(0, 0).unwrap());
        assert_eq!('b', *map2d.get(0, 1).unwrap());
        assert_eq!('c', *map2d.get(1, 0).unwrap());
        assert_eq!('d', *map2d.get(1, 1).unwrap());

        map2d.set(1, 0, 'x');
        assert_eq!('x', *map2d.get(1, 0).unwrap());

        assert_eq!(None, map2d.get(5, 0));
    }

    #[test]
    fn test_get_adjacent() {
        let map2d = &mut Map2d {
            data: vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ],
        };
        assert_eq!(0, map2d.get_adjacent(5, 2).len());

        let adj1 = map2d.get_adjacent(1, 1);
        assert_eq!(8, adj1.len());
        assert_eq!('b', *adj1[0]);
        assert_eq!('c', *adj1[1]);
        assert_eq!('f', *adj1[2]);
        assert_eq!('i', *adj1[3]);
        assert_eq!('h', *adj1[4]);
        assert_eq!('g', *adj1[5]);
        assert_eq!('d', *adj1[6]);
        assert_eq!('a', *adj1[7]);

        let adj2 = map2d.get_adjacent(0, 0);
        assert_eq!(3, adj2.len());
        assert_eq!('b', *adj2[0]);
        assert_eq!('e', *adj2[1]);
        assert_eq!('d', *adj2[2]);
    }

    #[test]
    fn test_len() {
        let map2d = &mut Map2d {
            data: vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ],
        };
        assert_eq!((3, 3), map2d.len())
    }
}
