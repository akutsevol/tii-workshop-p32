use std::fmt::{Debug, Formatter, Result as FmtResult};

pub trait Shape: Debug {
    fn name(&self) -> &str;
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
pub struct Point;

impl Shape for Point {
    fn name(&self) -> &str {
        "Point"
    }

    fn perimeter(&self) -> f32 {
        0.0
    }

    fn area(&self) -> f32 {
        0.0
    }

    fn scale(&mut self, _factor: f32) {
        // A point cannot be scaled.
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Shape for Triangle {
    fn name(&self) -> &str {
        "Triangle"
    }

    fn perimeter(&self) -> f32 {
        self.a + self.b + self.c
    }

    fn area(&self) -> f32 {
        // Using Heron's formula
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn scale(&mut self, factor: f32) {
        self.a *= factor;
        self.b *= factor;
        self.c *= factor;
    }
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Shape for Circle {
    fn name(&self) -> &str {
        "Circle"
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
    fn name(&self) -> &str {
        "Rectangle"
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

#[allow(dead_code)]
pub enum DynamicShape {
    Point(Point),
    Triangle(Triangle),
    Circle(Circle),
    Rectangle(Rectangle),
}

impl Debug for DynamicShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            DynamicShape::Point(p) => p.fmt(f),
            DynamicShape::Triangle(t) => t.fmt(f),
            DynamicShape::Circle(c) => c.fmt(f),
            DynamicShape::Rectangle(r) => r.fmt(f),
        }
    }
}

impl Shape for DynamicShape {
    fn name(&self) -> &str {
        match self {
            DynamicShape::Point(_) => "Point",
            DynamicShape::Triangle(_) => "Triangle",
            DynamicShape::Circle(_) => "Circle",
            DynamicShape::Rectangle(_) => "Rectangle",
        }
    }

    fn perimeter(&self) -> f32 {
        match self {
            DynamicShape::Point(shape) => shape.perimeter(),
            DynamicShape::Triangle(shape) => shape.perimeter(),
            DynamicShape::Circle(shape) => shape.perimeter(),
            DynamicShape::Rectangle(shape) => shape.perimeter(),
        }
    }

    fn area(&self) -> f32 {
        match self {
            DynamicShape::Point(shape) => shape.area(),
            DynamicShape::Triangle(shape) => shape.area(),
            DynamicShape::Circle(shape) => shape.area(),
            DynamicShape::Rectangle(shape) => shape.area(),
        }
    }

    fn scale(&mut self, factor: f32) {
        match self {
            DynamicShape::Point(shape) => shape.scale(factor),
            DynamicShape::Triangle(shape) => shape.scale(factor),
            DynamicShape::Circle(shape) => shape.scale(factor),
            DynamicShape::Rectangle(shape) => shape.scale(factor),
        }
    }
}

pub enum SliceChoice<'a> {
    First(&'a [Box<dyn Shape>]),
    Second(&'a [Box<dyn Shape>]),
}

pub fn biggest_perimeter_to_area_ratio<'a>(
    shapes1: &'a [Box<dyn Shape>],
    shapes2: &'a [Box<dyn Shape>],
) -> SliceChoice<'a> {
    let ratio1: f32 = shapes1.iter().map(|s| s.perimeter() / s.area()).sum();
    let ratio2: f32 = shapes2.iter().map(|s| s.perimeter() / s.area()).sum();

    if ratio1 > ratio2 {
        println!("Slice 1: {:?}", shapes1);
        SliceChoice::First(shapes1)
    } else {
        println!("Slice 2: {:?}", shapes2);
        SliceChoice::Second(shapes2)
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::*;

    #[test]
    fn test_shapes() {
        let mut circle = Circle { radius: 1.0 };
        let mut rectangle = Rectangle { width: 2.0, height: 3.0 };

        circle.print_properties();
        rectangle.print_properties();

        assert_eq!(circle.name(), "Circle");
        assert_eq!(rectangle.name(), "Rectangle");

        assert!((circle.perimeter() - 2.0 * std::f32::consts::PI).abs() < 1e-6);
        assert!((rectangle.perimeter() - 10.0).abs() < 1e-6);

        assert!((circle.area() - std::f32::consts::PI).abs() < 1e-6);
        assert!((rectangle.area() - 6.0).abs() < 1e-6);

        circle.scale(2.0);
        rectangle.scale(2.0);

        assert!((circle.perimeter() - 4.0 * std::f32::consts::PI).abs() < 1e-6);
        assert!((rectangle.perimeter() - 20.0).abs() < 1e-6);
    }

    #[test]
    fn test_biggest_perimeter_to_area_ratio() {
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
    }
}