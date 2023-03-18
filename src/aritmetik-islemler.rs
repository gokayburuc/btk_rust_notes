fn main() {
    // toplam();
    // us_alma();
    // mod_islemi();
    bitwise_logic();
}

fn toplam() {
    let mut sayi = 3 + 5 + 11;
    println!("{}", sayi);

    let b;
    b = sayi + 3333333;
    println!("{}", b);

    sayi += 1; // sayi ++ 1
    println!("sayi +1 = {}", sayi);
}

fn us_alma() {
    // power
    let us: isize = isize::pow(5, 10);
    println!("5 in 10 inci kuvveti = {}", us);

    // powi
    let yeni_sayi: f64 = f64::powi(5., 4);
    println!("{}", yeni_sayi);

    // powf
    let taban: f64 = 2.6;
    let diger_sayi: f64 = f64::powf(taban, std::f64::consts::PI);

    println!("powf: {}", diger_sayi);

    // print normal
    println!("{} üssü {} ={}", taban, std::f64::consts::PI, diger_sayi);

    // print with index
    println!(
        "{2} üssü {0} = {1}",
        std::f64::consts::PI,
        diger_sayi,
        taban
    );
}

fn mod_islemi() {
    // module
    println!("1024 mod 17 = {}", 1024 % 17);

    //
}

fn bitwise_logic() {
    // OR
    let c: i32 = 1 | 2;
    println!("1 | 2 = {}", c);

    // XOR ^

    // AND &

    // LOGICAL
    let mantik_sinama: bool = std::f64::consts::PI < 4.0; // true - false
    println!("PI sayisi 4 ten kucuk mu ? : {}", mantik_sinama);

    // == >= > < <= != operatörleri calisir

    let tutulan_sayi: isize = 17;
    println!("tutulan sayi 17 mi ? : {}", tutulan_sayi == 17); // true
}
