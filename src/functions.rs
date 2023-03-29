pub fn function_call() {
    another_function();
    deger_yaz(55);
    let mut z = 22;
    deger_arttir(&mut z);
    println!("z nin yeni degeri {}", z);

    let a = 5;
    let b = 8;
    println!("{} * {} = {}", a, b, carpma_islemi(a, b));
}

fn another_function() {
    println!("Basic Function")
}

fn deger_yaz(x: i32) {
    println!("girdiginiz deger: {}", x);
}

fn deger_arttir(x: &mut i32) {
    *x += 1;
}

// x y args and return i32
fn carpma_islemi(x: i32, y: i32) -> i32 {
    x * y
}
