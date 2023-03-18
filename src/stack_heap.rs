use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn orjin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_heap_call() {
    let p1: Point = orjin(); // orjin'e baglı bir pointer
    println!("p1 boyut olarak {} yer tutar", mem::size_of_val(&p1)); // 16 bit yer tutar

    let p2: Box<Point> = Box::new(orjin()); // bellekte box olarak yer tutar
    println!("p2 boyut olarak {} yer tutar", mem::size_of_val(&p2)); // 8 bit yer tutar

    let p3 = *p2; // p2 'nin degerinin getir
}
