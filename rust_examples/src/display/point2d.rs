use std::fmt;

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn example_output() {
    let pt = Point2D{x:0.0, y:14.0};

    println!("Display: {}", pt);
    println!("Debug: {:?}", pt);
    println!("Pretty print Debug: {:#?}", pt);
}
