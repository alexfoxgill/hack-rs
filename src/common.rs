use std::{fs::File, io::BufRead, path::Path};

pub type Error = Box<dyn std::error::Error>;
pub type Res<T = ()> = Result<T, Error>;

pub fn err(e: &str) -> Error {
    e.into()
}

pub fn read_lines(path: impl AsRef<Path>) -> Res<Vec<String>> {
    let file = File::open(path)?;
    let lines = std::io::BufReader::new(file).lines().map(|x| x.map_err(|e| e.into()));
    lines.collect()
}
