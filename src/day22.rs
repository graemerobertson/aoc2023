use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day22() {
    let f: File = File::open("data/day22.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let _lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
}
