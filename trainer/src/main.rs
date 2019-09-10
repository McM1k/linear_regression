mod parser;
mod draw;

use std::env;
use std::fs::File;
use std::path::Path;
use crate::parser::get_file_content;
use std::io::Write;

fn main() {
    let filename = env::args().nth(1).unwrap_or_else(|| panic!("Cannot read fileName"));
    let data = get_file_content(filename);

    let mut theta = (f64::from(0), f64::from(0));
    let rate = -1.0;
    for i in 0..(data.len() * 4) {
        let tmp0 = calc_t0(rate, data.clone(), theta);
        let tmp1 = calc_t1(rate, data.clone(), theta);
        println!("{} {}", theta.0, theta.1);
        theta = (theta.0 - tmp0, theta.1 - tmp1);
    }

    draw::render_graph(data);
    write_in_file("theta".to_string(), theta);
}

fn estimate_price(mileage: f64, theta: (f64, f64)) -> f64 {
    theta.0 + (theta.1 * mileage)
}

fn calc_t0(rate: f64, data: Vec<(usize, usize)>, theta: (f64, f64)) -> f64 {
    let mut sum = f64::from(0);
    for (k, p) in data.clone() {
        let (km, price) = (k as f64, p as f64);
        sum += estimate_price(km, theta) - price;
    }
    rate * (sum / data.len() as f64)
}

fn calc_t1(rate: f64, data: Vec<(usize, usize)>, theta: (f64, f64)) -> f64 {
    let mut sum = f64::from(0);
    for (k, p) in data.clone() {
        let (km , price) = (k as f64, p as f64);
        sum += (estimate_price(km, theta) - price) * km;
    }
    rate * (sum / data.len() as f64)
}

fn write_in_file(filename: String, theta: (f64, f64)) {
    let mut line = theta.0.to_string();
    line.push_str(" ");
    line.push_str(theta.1.to_string().as_str());

    let mut thetas = File::create(Path::new(&filename)).expect("cannot create theta file");
    match thetas.write_all(line.as_bytes()) {
        Ok(_x) => (),
        Err(_e) => panic!("Cannot write theta file"),
    }
}

#[cfg(test)]
mod trainer_tests {
    mod estimate_price {
        use crate::estimate_price;

        #[test]
        fn null_values() {
            let test = estimate_price(0.0, (0.0, 0.0));
            assert_eq!(test, 0.0);
        }

        #[test]
        fn null_values_with_mileage() {
            let test = estimate_price(10000.0, (0.0, 0.0));
            assert_eq!(test, 0.0);
        }

        #[test]
        fn with_values_17() {
            let test = estimate_price(4.0, (5.0, 3.0));
            assert_eq!(test, 17.0);
        }
    }

    mod calc_t0 {
        use crate::calc_t0;

        #[test]
        fn with_values_17() {
            let data = vec![(4, 15)];
            let test = calc_t0(1.0, data, (5.0, 3.0));
            assert_eq!(test, 2.0);
        }

        #[test]
        fn with_values_17_next() {
            let data = vec![(4, 15)];
            let test = calc_t0(1.0, data, (2.0, 8.0));
            assert_eq!(test, 19.0);
        }
    }

    mod calc_t1 {
        use crate::calc_t1;

        #[test]
        fn with_values_17() {
            let data = vec![(4, 15)];
            let test = calc_t1(1.0, data, (5.0, 3.0));
            assert_eq!(test, 8.0);
        }

        #[test]
        fn with_values_17_next() {
            let data = vec![(4, 15)];
            let test = calc_t1(1.0, data, (2.0, 8.0));
            assert_eq!(test, 76.0);
        }
    }
}