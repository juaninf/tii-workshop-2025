use super::*;

#[test]
fn test_circle_area() {
    let center = Point::new(0, 0);
    let radius = 3;
    let circle = Circle::new(center, radius);
    assert_eq!(
        circle.area(),
        std::f64::consts::PI * (radius as f64).powi(2)
    );
}

#[test]
fn test_biggest_area() {
    let center1 = Point::new(0, 0);
    let radius1 = 3;
    let circle1 = Circle::new(center1, radius1);

    let center2 = Point::new(0, 0);
    let radius2 = 4;
    let circle2 = Circle::new(center2, radius2);

    let circle = circle1.biggest_area(&circle2);
    assert_eq!(circle.area(), circle2.area());
}

#[test]
fn test_test() {
    let center1 = Point::new(0, 0);
    let radius1 = 3;
    let circle1 = Circle::new(center1, radius1);
    let s = String::from("Hello");
    let result = circle1.test(s);
    assert_eq!(result, "Hello");
}

#[test]
fn test_compute_biggest_area_from_two_slices() {
    let center1 = Point::new(0, 0);
    let radius1 = 3;
    let circle1 = Circle::new(center1, radius1);

    let pointx = Point::new(20, 30);
    let pointy = Point::new(0, 0);
    let rectangle = Rectangle::new(pointx, pointy);

    let shapes1: [Circle; 1] = [circle1];
    let shapes2: [Rectangle; 1] = [rectangle];

    let result = compute_biggest_area_from_two_slices(&shapes1, &shapes2);
    match result {
        Some(GenericShape::One(shape)) => {
            println!("Circle is begger");
            assert_eq!(shape.area_to_perimeter(), 0f64);
        }
        Some(GenericShape::Two(shape)) => {
            println!("Rect is begger");
            assert_eq!(shape.area_to_perimeter(), 138.46153846153845f64);
        }
        None => {
            println!("Hi")
        }
    }
}
