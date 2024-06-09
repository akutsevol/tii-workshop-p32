mod shapes;

fn main() {
    use shapes::*;

    let shapes1: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
    ];

    let shapes2: Vec<Box<dyn Shape>> = vec![
        Box::new(Triangle { a: 3.0, b: 4.0, c: 5.0 }),
        Box::new(Point),
    ];

    let result = biggest_perimeter_to_area_ratio(&shapes1, &shapes2);

    match result {
        SliceChoice::First(_) => println!("Slice 1 has the biggest perimeter to area ratio"),
        SliceChoice::Second(_) => println!("Slice 2 has the biggest perimeter to area ratio"),
    }

    for shape in shapes1.iter().chain(shapes2.iter()) {
        shape.print_properties();
        println!("Area to Perimeter Ratio: {}", shape.area_to_perimeter());
    }
}
