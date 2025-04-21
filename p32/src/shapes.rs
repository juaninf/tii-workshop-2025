use std::any::type_name;

struct Point {
    x: i32,
    y: i32,
}
struct Circle {
    center_point: Point,
    radius: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

enum DynamicShape {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
    Point(Point),
}

enum GenericShape<T, L> {
    One(T),
    Two(L),
}

trait Shape {
    const NAME: &'static str;
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn scale(self, factor: f32) -> Self;
    fn area_to_perimeter(&self) -> f64;
    fn print_properties(&self);

    fn biggest_area<'a>(&'a self, second_shape: &'a Self) -> &'a Self {
        if self.area() > second_shape.area() {
            self
        } else {
            &second_shape
        }
    }
}

fn compute_biggest_area_from_two_slices<'a, T: Shape, L: Shape>(
    x: &'a [T],
    y: &'a [L],
) -> Option<GenericShape<&'a T, &'a L>> {
    let mut T_biggest_area = 0.0;
    let mut L_biggest_area = 0.0;
    let mut T_biggest_shape: &T = &x[0];
    let mut L_biggest_shape: &L = &y[0];

    let mut biggest_shape = None;

    for shape in x.iter() {
        let rtio = shape.area_to_perimeter();
        if rtio > T_biggest_area {
            biggest_shape = Some(GenericShape::One(shape))
        }
    }

    for shape in y.iter() {
        let rtio = shape.area_to_perimeter();
        if rtio > L_biggest_area {
            biggest_shape = Some(GenericShape::Two(shape))
        }
    }

    if T_biggest_area > L_biggest_area {
        Some(GenericShape::One(T_biggest_shape))
    } else {
        Some(GenericShape::Two(L_biggest_shape))
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64
    }
}

impl Circle {
    fn new(center_point: Point, radius: i32) -> Self {
        Circle {
            center_point,
            radius,
        }
    }
}

impl Rectangle {
    fn new(a: Point, b: Point) -> Self {
        Rectangle { a, b }
    }
}

impl Shape for Rectangle {
    const NAME: &'static str = "Rectangle";
    fn area(&self) -> f64 {
        let width = self.a.distance(&Point::new(self.b.x, self.a.y));
        let height = self.a.distance(&Point::new(self.a.x, self.b.y));
        width * height
    }

    fn perimeter(&self) -> f64 {
        let width = self.a.distance(&Point::new(self.b.x, self.a.y));
        let height = self.a.distance(&Point::new(self.a.x, self.b.y));
        2.0 * (width + height)
    }

    fn scale(self, factor: f32) -> Self {
        Rectangle {
            a: self.a,
            b: Point {
                x: (self.b.x as f32 * factor) as i32,
                y: (self.b.y as f32 * factor) as i32,
            },
        }
    }

    fn area_to_perimeter(&self) -> f64 {
        self.area() / self.perimeter()
    }

    fn print_properties(&self) {
        println!(
            "Rectangle: a=({},{}) b=({},{}) area={} perimeter={}",
            self.a.x,
            self.a.y,
            self.b.x,
            self.b.y,
            self.area(),
            self.perimeter()
        );
    }
}

impl Triangle {
    fn new(a: Point, b: Point, c: Point) -> Self {
        Triangle { a, b, c }
    }
}

impl Shape for Triangle {
    const NAME: &'static str = "Triangle";

    fn area(&self) -> f64 {
        let a = self.a.distance(&self.b);
        let b = self.b.distance(&self.c);
        let c = self.c.distance(&self.a);
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a.distance(&self.b) + self.b.distance(&self.c) + self.c.distance(&self.a)
    }

    fn scale(self, factor: f32) -> Self {
        Triangle {
            a: self.a,
            b: Point {
                x: (self.b.x as f32 * factor) as i32,
                y: (self.b.y as f32 * factor) as i32,
            },
            c: Point {
                x: (self.c.x as f32 * factor) as i32,
                y: (self.c.y as f32 * factor) as i32,
            },
        }
    }

    fn area_to_perimeter(&self) -> f64 {
        self.area() / self.perimeter()
    }

    fn print_properties(&self) {
        println!(
            "Triangle: a=({},{}) b=({},{}) c=({},{}) area={} perimeter={}",
            self.a.x,
            self.a.y,
            self.b.x,
            self.b.y,
            self.c.x,
            self.c.y,
            self.area(),
            self.perimeter()
        );
    }
}

impl Shape for Circle {
    const NAME: &'static str = "Circle";

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius as f64).powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius as f64
    }

    fn scale(self, factor: f32) -> Self {
        Circle {
            center_point: self.center_point,
            radius: (self.radius as f32 * factor) as i32,
        }
    }

    fn area_to_perimeter(&self) -> f64 {
        self.area() / self.perimeter()
    }

    fn print_properties(&self) {
        println!(
            "Circle: center=({},{}) radius={} area={} perimeter={}",
            self.center_point.x,
            self.center_point.y,
            self.radius,
            self.area(),
            self.perimeter()
        );
    }
}

fn compare_shapes() {
    println!("Comparing shapes:");
}

impl Circle {
    fn test(&self, s: String) -> String {
        println!("Circle test: {}", s);
        s
    }
}

#[cfg(test)]
mod shapes_tests;
