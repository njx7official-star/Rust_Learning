#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

fn largest_shape(v: &Vec<Shape>) {
    let mut area = [0.0; 3];

    v.iter().for_each(|v| match v {
        Shape::Circle(radius) => area[0] = std::f64::consts::PI * radius * radius,
        Shape::Square(length) => area[1] = length * length,
        Shape::Triangle(base, height) => area[2] = 0.5 * base * height,
    });

    println!("{:?}", area);
    match area.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
        Some(&value) if value == area[0] => println!("largest area circle"),
        Some(&value) if value == area[1] => println!("largest area Square"),
        Some(&value) if value == area[2] => println!("largest area Triangle"),
        _ => println!("Error occured"),
    };
}

fn main() {
    let mut shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle(2.0, 4.0),
    ];

    largest_shape(&shapes);

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
