use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::path::Path;

pub fn file_read_lines<P>(file: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file)?;
    Ok(BufReader::new(file).lines())
}
