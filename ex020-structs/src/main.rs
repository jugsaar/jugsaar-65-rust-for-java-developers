// Defines the struct Point
// #[derive(Debug)] // implements Debug trait
struct Point {
    x: i32,
    y: i32,
}

// Defines Method implementations for Point
impl Point {
    // Method using 'self'
    fn print_coordinates(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    p.print_coordinates(); // Output: x: 10, y: 20

    // print!("p = {:?}", p); // print "Debug" representation
}
