pub trait Shape {
    fn name(&self) -> String;
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
    fn scale(&mut self, factor: f32);
    fn area_to_perimeter(&self) -> f32 {
        self.area() / self.perimeter()
    }

    fn print_properties(&self) {
        println!("Name: {}", self.name());
        println!("Area: {}", self.area());
        println!("Perimeter: {}", self.perimeter());
    }
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Shape for Circle {
    fn name(&self) -> String {
        "Circle".to_string()
    }

    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    fn scale(&mut self, factor: f32) {
        self.radius *= factor;
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Shape for Rectangle {
    fn name(&self) -> String {
        "Rectangle".to_string()
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }

    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: f32) {
        self.width *= factor;
        self.height *= factor;
    }
}

pub fn biggest_area<'a, S1, S2>(shape1: &'a S1, shape2: &'a S2) -> &'a dyn Shape
where
    S1: Shape,
    S2: Shape,
{
    if shape1.area() > shape2.area() {
        shape1
    } else {
        shape2
    }
}