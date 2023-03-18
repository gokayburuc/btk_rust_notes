use std::mem;

fn main() {
    let a: u8 = 129; // unsigned 8 bit value max 0-256
    println!("a :{}", a);

    let mut b: i8 = 9; // -128  ... +127
    println!("b :{}", b);

    b = 22; // b changed
    println!("b :{}", b);

    let c: i32 = 12731; // i32 boyutundan daha büyük bir deger
    println!("c :{} boyutu :{} byte", c, mem::size_of_val(&c));

    // println!("a : {}", mem::size_of_val())

    let d: isize = -200;
    let d_size: usize = mem::size_of_val(&d);
    println!(
        "d : {} boyut: {} bilgisayar mimarisi : {}",
        d,
        d_size,
        d_size * 8
    );

    let e: char = 't'; // signs + numbers + letters  = charachter
    println!(" e : {}  boyut : {} ", e, mem::size_of_val(&e));

    // float - f32 ve f64
    // u olamaz

    let f: f32 = 34.5;
    println!(" f : {}  boyut : {} ", f, mem::size_of_val(&f));

    // bool
    //
    let g: bool = true; // false
    println!("g: {} boyut : {}", g, mem::size_of_val(&g));
}
