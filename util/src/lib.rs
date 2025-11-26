use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Result},
    path::Path,
};

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
