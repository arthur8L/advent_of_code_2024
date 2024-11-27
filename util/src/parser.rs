use std::{fs::File, io::Read};

pub struct AdventParser;

impl AdventParser {
    pub fn read(path: std::path::PathBuf) -> std::io::Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn lines<'a>(src: &'a str) -> Vec<&'a str> {
        src.lines().collect::<Vec<&'a str>>()
    }
}
