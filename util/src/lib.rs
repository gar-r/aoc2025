use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
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
