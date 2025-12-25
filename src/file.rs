use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn to_reader(path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    Ok(std::io::BufReader::new(file))
}
