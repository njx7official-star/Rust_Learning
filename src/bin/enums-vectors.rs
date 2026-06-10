enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle(2.0, 4.0),
    ];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        })
        .sum();

    println!("Total area: {} sq. units", total_area);
}
