mod parser;
use std::env;
use std::fs::File;
use std::path::Path;
use crate::parser::get_file_content;
use std::io::Write;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| panic!("Cannot read fileName"));
    let data = get_file_content(filename);

    let mut t0 = f64::from(0);
    let mut t1 = f64::from(0);
    let rate = -1.0;
    for i in 0..(data.len() * 4) {
        let tmp0 = calc_t0(rate, data.clone(), t0, t1);
        let tmp1 = calc_t1(rate, data.clone(), t0, t1);
        println!("{} {}", t0, t1);
        t0 = tmp0;
        t1 = tmp1;
    }

    write_in_file("theta".to_string(), t0, t1);
}

fn write_in_file(filename: String, t0: f64, t1: f64) {
    let mut line = t0.to_string();
    line.push_str(" ");
    line.push_str(t1.to_string().as_str());

    let mut thetas = File::create(Path::new(&filename)).expect("cannot create theta file");
    match thetas.write_all(line.as_bytes()) {
        Ok(_x) => (),
        Err(_e) => panic!("Cannot write theta file"),
    }
}

fn estimate_price(mileage: f64, t0: f64, t1: f64) -> f64 {
    t0 + (t1 * mileage)
}

fn calc_t0(rate: f64, data: Vec<(usize, usize)>, tmp0: f64, tmp1: f64) -> f64 {
    let mut sum = f64::from(0);
    for (k, p) in data.clone() {
        let (km, price) = (k as f64, p as f64);
        sum += estimate_price(km, tmp0, tmp1) - price;
    }
    rate * (sum / data.len() as f64)
}

fn calc_t1(rate: f64, data: Vec<(usize, usize)>, tmp0: f64, tmp1: f64) -> f64 {
    let mut sum = f64::from(0);
    for (k, p) in data.clone() {
        let (km , price) = (k as f64, p as f64);
        sum += (estimate_price(km, tmp0, tmp1) - price) * km;
    }
    rate * (sum / data.len() as f64)
}