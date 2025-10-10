enum Direction {
    East,
    West,
    North,
    South,
}

enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32, f32),
    Square(f32),
}

fn main() {
    // Enums
    let direction = Direction::East;
    let new_direction = direction;
    move_around(new_direction);

    // Enums with values
    let rect = Shape::Rectangle(3.0, 4.0);
    let circle = Shape::Circle(3.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);
    let square = Shape::Square(3.0);

    let area = calculate_area(rect);
    println!("Area of rectangle: {}", area);

    let area = calculate_area(circle);
    println!("Area of circle: {}", area);

    let area = calculate_area(triangle);
    println!("Area of triangle: {}", area);

    let area = calculate_area(square);
    println!("Area of square: {}", area);

    // Option enum
    let string = String::from("Arshdeep");

    match find_first_a(string) {
        Some(index) => {
            println!("Index of first a: {}", index);
        }
        None => {
            println!("No a found");
        }
    }
}

fn calculate_area(shape: Shape) -> f32 {
    // pattern matching
    match shape {
        Shape::Circle(radius) => radius * radius * std::f32::consts::PI,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle(base, height, _f32) => 0.5 * base * height,
        Shape::Square(side) => side * side,
    }
}

fn find_first_a(string: String) -> Option<i32> {
    for (i, char) in string.chars().enumerate() {
        if char == 'a' {
            return Some(i as i32);
        }
    }
    return None;
}

fn move_around(direction: Direction) {
    // implement logic to move around
}
