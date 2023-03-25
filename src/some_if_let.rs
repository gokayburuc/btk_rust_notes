pub fn some_code() {
    let x = 2.0;
    let y = 0.0;

    // option -> Some(c) | None
    //

    let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Sıfıra bölme durumu"),
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }
}
