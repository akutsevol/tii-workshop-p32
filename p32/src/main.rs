mod shapes;

fn main() {
    use shapes::{biggest_area, Circle, Rectangle, Shape};

    let mut circle = Circle { radius: 5.0 };
    let mut rectangle = Rectangle { width: 4.0, height: 6.0 };

    circle.print_properties();
    rectangle.print_properties();

    circle.scale(2.0);
    rectangle.scale(5.5);

    println!("\nAfter scaling:");
    circle.print_properties();
    rectangle.print_properties();

    let biggest = biggest_area(&circle, &rectangle);
    println!("\nShape with the biggest area:");
    biggest.print_properties();
}
