use std::{fs::File, io::{BufReader, BufRead}};

use crate::ErrorKind;

#[inline]
pub fn buffer_read(file: &File) -> String {
    BufReader::new(file)
        .lines()
        .map(|f| f.unwrap())
        .collect::<Vec<String>>()
        .join("\n")
}

#[inline]
pub fn open_file(path: &str) -> Result<File, ErrorKind> {
    Ok(File::open(path).or_else(|err| Err(ErrorKind::FileErr(err)))?)
}
