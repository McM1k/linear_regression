/*
use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;

pub fn render_graph(data: &[(usize, usize)]) {
    let mut dataf64: [(f64, f64)];
    for (x, y) in data{
        dataf64.push((*x as f64, *y as f64));
    }

    let s1 = Scatter::from_slice(&dataf64)
        .style(scatter::Style::new()
            .marker(Marker::Square)
            .colour("#DD3355"));

    let v = View::new()
        .add(&s1)
        .x_label("mileage")
        .y_label("price");

    Page::single(&v).save("graph.svg");
}

*/

use gnuplot::{Figure};

pub fn render_graph(data: Vec<(usize, usize)>) {
    let x_data : Vec<usize> = data.iter().map(|(x, _y)| *x).collect();
    let y_data : Vec<usize> = data.iter().map(|(_x, y)| *y).collect();

    let mut fg = Figure::new();
    fg.axes2d().points(x_data, y_data,&[]);
    fg.show();
}