pub fn strings_call() {
    // static UTF-8
    let s: &'static str = "merhaba";

    // immutable
    // s = "selam";
    //

    // let b = s[0];

    for c in s.chars() {
        println!("{}", c);
    }

    if let Some(ilk_karakter) = s.chars().nth(6) {
        println!("ilk karakter: {}", ilk_karakter);
    }

    // heap
    let mut karakterler = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8) {
        karakterler.push(a as char);
        karakterler.push_str(",");
        a += 1;
    }

    println!("{}", karakterler);

    // str to String
    let u: &str = &karakterler;

    //string + str
    // let mut abc = "Merhaba Dunya";
    // let mut abc = String::from("Merhaba Dünya");
    let mut abc = "Merhaba Dunya".to_string();

    abc.remove(0);
    println!("{}", abc);

    let new_abc = abc.replace("a", "c");
    println!("{}", new_abc);

    abc.push_str("!!!");
    println!("{}", abc);

    //string + &String
}
