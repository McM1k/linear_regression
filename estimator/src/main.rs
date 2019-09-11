#[macro_use] extern crate scan_fmt;
use std::fs::File;
use std::io::Read;

fn open_file() -> File {
    File::open("../trainer/theta").expect("Cannot open theta file")
}

fn parse_thetas(mut file : File) -> (f64, f64) {
    let mut content= String::new();
    file.read_to_string(&mut content).expect("Cannot read to a string");

    let mut iter = content.split_whitespace();
    let t0 = iter.next().unwrap_or_else(|| panic!("Cannot read t0")).parse::<f64>().expect("Cannot parse t0");
    let t1 = iter.next().unwrap_or_else(|| panic!("Cannot read t1")).parse::<f64>().expect("Cannot parse t1");

    (t0, t1)
}

fn get_thetas() -> (f64, f64) {
    let file = open_file();
    parse_thetas(file)
}

fn main() {
    println!("Please input mileage : ");
    let mileage = match scanln_fmt!("{}", usize) {
        Ok(x) => x,
        Err(_e) => panic!("Wrong input"),
    };

    let (t0, t1) = get_thetas();

    let estimation = t0 + (t1 * mileage as f64);
    println!("Estimated price is : {}", estimation);
}
