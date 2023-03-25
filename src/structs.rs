struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

pub fn structs() {
    let p = Point { x: 4.0, y: 3.0 };
    println!("point p is ({}{})", p.x, p.y);
    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
    println!(
        "my line ({} {}) - ({} {})",
        myline.start.x, myline.start.y, myline.end.x, myline.end.y
    );
}
