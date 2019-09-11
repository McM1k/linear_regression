use plotlib::line;
use plotlib::page::Page;
use plotlib::scatter;
use plotlib::scatter::Scatter;
use plotlib::style::{Line, Marker, Point};
use plotlib::view::ContinuousView;

pub fn render_graph(data: Vec<(usize, usize)>, xrange: (f64, f64), theta: (f64, f64)) {
    let mut dataf64 = Vec::new();
    for (x, y) in data.clone() {
        dataf64.push((x as f64, y as f64));
    }

    let s1 = Scatter::from_slice(&dataf64).style(
        scatter::Style::new()
            .marker(Marker::Square)
            .colour("#DD3355"),
    );

    let p1 = (xrange.0, xrange.0 * theta.1 + theta.0);
    let p2 = (xrange.1, xrange.1 * theta.1 + theta.0);

    let l1 = line::Line::new(&[p1, p2]).style(line::Style::new().colour("#3355DD").width(4.0));

    let v = ContinuousView::new()
        .add(&s1)
        .add(&l1)
        .x_label("mileage")
        .y_label("price");

    Page::single(&v).save("graph.svg").expect("Cannot draw svg");
    println!("Succesfully created trainer/graph.svg !");
}
