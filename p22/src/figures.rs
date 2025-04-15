pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x).pow(2);
        let dy = (self.y - other.y).pow(2);
        ((dx + dy) as f64).sqrt()
    }
}

impl Triangle {
    pub fn perimeter(&self) -> f64 {
        let a = self.a.distance(&self.b);
        let b = self.b.distance(&self.c);
        let c = self.c.distance(&self.a);
        a + b + c
    }

    pub fn area(&self) -> f64 {
        let a = self.a.distance(&self.b);
        let b = self.b.distance(&self.c);
        let c = self.c.distance(&self.a);
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}

pub struct Circle {
    center_point: Point,
    radius: i32,
}

impl Circle {
    /// compute the area of the circle
    /// # Examples
    /// ```
    /// use p22::figures::Point;
    /// let center = Point::new(0, 0);
    /// ```
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius as f64).powi(2)
    }

    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius as f64
    }
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        let width = self.a.distance(&Point::new(self.b.x, self.a.y));
        let height = self.a.distance(&Point::new(self.a.x, self.b.y));
        width * height
    }

    pub fn perimeter(&self) -> f64 {
        let width = self.a.distance(&Point::new(self.b.x, self.a.y));
        let height = self.a.distance(&Point::new(self.a.x, self.b.y));
        2.0 * (width + height)
    }
}

struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

struct Rectangle {
    a: Point,
    b: Point,
}

enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[cfg(test)]
mod figures_tests;
