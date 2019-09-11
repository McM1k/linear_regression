use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn get_file_content(filename: String) -> Vec<(usize, usize)> {
    let file = open_file(filename);
    let lines = file_to_strings(file);

    let mut data = vec![];
    for line in lines.iter().skip(1) {
        data.push(parse_line(line.clone()));
    }
    data
}

fn parse_line(line: String) -> (usize, usize) {
    let vec: Vec<String> = line.split(',').map(|l| l.to_string()).collect();
    if vec.len() != 2 {
        panic!("Data line corrupted");
    }

    let km = vec[0].parse::<usize>().expect("Cannot parse km");
    let price = vec[1].parse::<usize>().expect("Cannot parse price");
    (km, price)
}

fn file_to_strings(file: File) -> Vec<String> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Cannot read from file");
    contents.lines().map(|l| l.to_string()).collect()
}

fn open_file(filename: String) -> File {
    File::open(filename).expect("Cannot open file")
}
