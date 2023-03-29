fn merhaba() {
    println!("Selamın Aleykum!")
}

pub fn closure_call() {
    let selam = merhaba;
    selam();

    // bir ekle isimli bir lambda fonksiyonu olusturduk bknz.python
    let bir_ekle = |x: i32| -> i32 { x + 1 };

    let a = 8;
    println!("{}+1 = {}", a, bir_ekle(a));

    let b = 2;
    {
        let iki_ekle = |x| {
            let mut z = x;
            z += b;
            z
        };
        println!("{} + 2 = {} ", 22, iki_ekle(22));
    }

    let odunc_al = &b;
    println!("{}", odunc_al);

    let arti_uc: fn(i32) = |mut _x: i32| _x += 3;
    let mut k: i32 = 14;
    arti_uc(k);
    println!("k = {}", k);
}
