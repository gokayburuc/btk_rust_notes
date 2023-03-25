pub fn matchstring_call() {
    for x in 0..13 {
        println!("{}: benim {} elmam var", x, ne_kadar(x));
    }

    let point:(i32,i32) = (3, 0);

}

// string static veri döndurur
fn ne_kadar(x: i32) -> &'static str {
    match x {
        0 => "hic",
        1 | 2 => "bir veya iki",
        12 => "bir düzine",
        z @ 6.=8 => "6 ile 8 arasi",
        _ if (x % 2 == 0) => "cift sayida",
        _ => "biraz",
    }
}
