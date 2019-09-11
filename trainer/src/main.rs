mod draw;
mod parser;

use crate::parser::get_file_content;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("Cannot read fileName"));
    let data = get_file_content(filename);
    let xrange = get_range(data.clone());
    let scaled_data = feature_scaling(data.clone(), xrange);

    let mut theta = (f64::from(0), f64::from(0));
    let rate = 1.0;
    let iterations = data.len() * 20;
    for _i in 0..iterations {
        let tmp0 = calc_t0(rate, scaled_data.clone(), theta);
        let tmp1 = calc_t1(rate, scaled_data.clone(), theta);
        //println!("{} {}", theta.0, theta.1);
        theta = (theta.0 - tmp0, theta.1 - tmp1);
    }
    theta = unnormalize(xrange, theta);
    draw::render_graph(data, xrange, theta);
    write_in_file("theta".to_string(), theta);
    println!("Thetas written in trainer/theta !");
}

fn get_range(data: Vec<(usize, usize)>) -> (f64, f64) {
    let mut xmax = 0;
    for (x, _y) in data.clone() {
        if xmax < x {
            xmax = x
        }
    }
    let mut xmin = xmax;
    for (x, _y) in data.clone() {
        if xmin > x {
            xmin = x
        }
    }

    (xmin as f64, xmax as f64)
}

fn unnormalize(xrange: (f64, f64), theta: (f64, f64)) -> (f64, f64) {
    (theta.0, theta.1 / (xrange.1 - xrange.0))
}

fn feature_scaling(data: Vec<(usize, usize)>, xrange: (f64, f64)) -> Vec<(f64, f64)> {
    data.iter()
        .map(|(x, y)| ((*x as f64 - xrange.0) / (xrange.1 - xrange.0), *y as f64))
        .collect()
}

fn estimate_price(mileage: f64, theta: (f64, f64)) -> f64 {
    theta.0 + (theta.1 * mileage)
}

fn calc_t0(rate: f64, data: Vec<(f64, f64)>, theta: (f64, f64)) -> f64 {
    let mut sum = f64::from(0);
    for (k, p) in data.clone() {
        let (km, price) = (k as f64, p as f64);
        sum += estimate_price(km, theta) - price;
    }
    rate * (sum / data.len() as f64)
}

fn calc_t1(rate: f64, data: Vec<(f64, f64)>, theta: (f64, f64)) -> f64 {
    let mut sum = f64::from(0);
    for (k, p) in data.clone() {
        let (km, price) = (k as f64, p as f64);
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
            let data = vec![(4.0, 15.0)];
            let test = calc_t0(1.0, data, (5.0, 3.0));
            assert_eq!(test, 2.0);
        }

        #[test]
        fn with_values_17_next() {
            let data = vec![(4.0, 15.0)];
            let test = calc_t0(1.0, data, (2.0, 8.0));
            assert_eq!(test, 19.0);
        }
    }

    mod calc_t1 {
        use crate::calc_t1;

        #[test]
        fn with_values_17() {
            let data = vec![(4.0, 15.0)];
            let test = calc_t1(1.0, data, (5.0, 3.0));
            assert_eq!(test, 8.0);
        }

        #[test]
        fn with_values_17_next() {
            let data = vec![(4.0, 15.0)];
            let test = calc_t1(1.0, data, (2.0, 8.0));
            assert_eq!(test, 76.0);
        }
    }
}
