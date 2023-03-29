pub fn method_call() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let my_line = Line { start: p, end: p2 };

    println!("mesafe {}", my_line.len());
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

// method here
impl Line {
    // this is a method
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;

        (dx * dx + dy * dy).sqrt()
    }
}
