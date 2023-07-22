// Define a simple trait
trait Shape {
    fn area(&self) -> f64;
}

// Implement the trait for a struct representing a circle
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Implement the trait for a struct representing a rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Function that takes a trait object as an argument and calls its methods
fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 3.0,
    };

    // Call the function with different concrete types that implement the Shape trait
    print_area(&circle);
    print_area(&rectangle);
}
