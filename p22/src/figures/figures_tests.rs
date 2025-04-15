use super::*;

#[test]
fn test_default_point() {
    let p = Point::default();
    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
}

#[test]
fn test_default_point_with_new() {
    let p = Point::new(0, 0);

    assert_eq!(p.x, 0);
    assert_eq!(p.y, 0);
    let mut pn = Point { x: 2, y: 2 };
}

#[test]
fn test_perimeter_triangle() {
    let a = Point::new(0, 0);
    let b = Point::new(3, 0);
    let c = Point::new(3, 4);
    let triangle = Triangle { a, b, c };
    assert_eq!(triangle.perimeter(), 12f64);
}

#[test]
fn test_area_triangle() {
    let a = Point::new(0, 0);
    let b = Point::new(3, 0);
    let c = Point::new(3, 4);
    let triangle = Triangle { a, b, c };
    assert_eq!(triangle.area(), 6f64);
}

#[test]
fn test_area_circle() {
    let center = Point::new(0, 0);
    let radius = 3;
    let circle = Circle {
        center_point: center,
        radius,
    };
    assert_eq!(
        circle.area(),
        std::f64::consts::PI * (radius as f64).powi(2)
    );
}

#[test]
fn test_circumference_circle() {
    let center = Point::new(0, 0);
    let radius = 3;
    let circle = Circle {
        center_point: center,
        radius,
    };
    assert_eq!(
        circle.circumference(),
        2.0 * std::f64::consts::PI * radius as f64
    );
}

#[test]
fn test_area_rectangle() {
    let a = Point::new(0, 0);
    let b = Point::new(3, 4);
    let rectangle = Rectangle { a, b };
    assert_eq!(rectangle.area(), 12f64);
}

#[test]
fn test_perimeter_rectangle() {
    let a = Point::new(0, 0);
    let b = Point::new(3, 4);
    let rectangle = Rectangle { a, b };
    assert_eq!(rectangle.perimeter(), 14f64);
}
