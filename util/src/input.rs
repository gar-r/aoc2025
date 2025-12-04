use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Result},
    path::Path,
};

use crate::map2d::Map2d;

pub fn read_input_lines<P>(path: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    return reader.lines().collect();
}

pub fn read_input_string<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(String::from(contents.trim_end()))
}

pub fn read_input_chars<P>(path: P) -> Result<Vec<char>>
where
    P: AsRef<Path>,
{
    let input_str = read_input_string(path)?;
    Ok(input_str.chars().collect())
}

pub fn read_input_map2d<P>(path: P) -> Result<Map2d>
where P: AsRef<Path>
{
    let mut map2d = Map2d { data: Vec::new() };
    let lines = read_input_lines(path)?;
    lines.iter().for_each(|l| map2d.data.push(l.chars().collect()));
    Ok(map2d)
}

#[cfg(test)]
mod tests {
    use crate::input::{read_input_chars, read_input_lines, read_input_map2d, read_input_string};

    #[test]
    fn test_read_input_lines() {
        let lines = read_input_lines("test_lines.txt").unwrap();
        assert_eq!(3, lines.len());
        assert_eq!("foo", lines[0]);
        assert_eq!("bar", lines[1]);
        assert_eq!("baz", lines[2]);
    }

    #[test]
    fn test_read_input_string() {
        let str = read_input_string("test_chars.txt").unwrap();
        assert_eq!("abcd", str);
    }

    #[test]
    fn test_read_input_chars() {
        let chars = read_input_chars("test_chars.txt").unwrap();
        assert_eq!(4, chars.len());
        assert_eq!('a', chars[0]);
        assert_eq!('b', chars[1]);
        assert_eq!('c', chars[2]);
        assert_eq!('d', chars[3]);
    }

    #[test]
    fn test_read_input_map2d() {
        let map2d = read_input_map2d("test_lines.txt").unwrap();
        assert_eq!(3, map2d.data.len());
        assert_eq!(3, map2d.data[0].len());
        assert_eq!(3, map2d.data[1].len());
        assert_eq!(3, map2d.data[2].len());
        assert_eq!('f', map2d.data[0][0]);
        assert_eq!('o', map2d.data[0][1]);
        assert_eq!('o', map2d.data[0][2]);
        assert_eq!('b', map2d.data[1][0]);
        assert_eq!('a', map2d.data[1][1]);
        assert_eq!('r', map2d.data[1][2]);
        assert_eq!('b', map2d.data[2][0]);
        assert_eq!('a', map2d.data[2][1]);
        assert_eq!('z', map2d.data[2][2]);
    }

}
