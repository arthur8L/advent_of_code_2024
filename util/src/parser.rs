use std::{
    fs::File,
    io::Read,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
pub struct AdventInput(String);

impl AdventInput {
    pub fn read(path: std::path::PathBuf) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(Self(content))
    }

    pub fn lines(&self) -> Vec<&str> {
        self.0.lines().collect::<Vec<&str>>()
    }

    pub fn split_column(&self) -> (Vec<&str>, Vec<&str>) {
        let lines = self.lines();
        lines.iter().fold(
            (
                Vec::with_capacity(lines.len()),
                Vec::with_capacity(lines.len()),
            ),
            |mut accu, l| {
                let cols: Vec<&str> = l.split_whitespace().collect();
                accu.0.push(cols[0]);
                accu.1.push(cols[1]);
                accu
            },
        )
    }

    pub fn split_column_as_i64(&self) -> (Vec<i64>, Vec<i64>) {
        let lines = self.lines();
        lines.iter().fold(
            (
                Vec::with_capacity(lines.len()),
                Vec::with_capacity(lines.len()),
            ),
            |mut accu, l| {
                let cols: Vec<&str> = l.split_whitespace().collect();
                accu.0.push(cols[0].parse::<i64>().unwrap());
                accu.1.push(cols[1].parse::<i64>().unwrap());
                accu
            },
        )
    }
}

impl Deref for AdventInput {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AdventInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
