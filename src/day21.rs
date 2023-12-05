use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn day21() {
    let f: File = File::open("data/day21.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let _lines = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
}
