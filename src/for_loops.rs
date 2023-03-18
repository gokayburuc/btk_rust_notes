use std::iter::Enumerate;

pub fn for_call() {
    // sistem mesajı
    println!("for dongusu calisiyor.\n");

    for x in 1..11 {
        //continue ile  deger atlama
        if x == 3 {
            continue;
        }
        println! {"{:?} Missisipi", x};

        // break ile loop durdurma
        if x == 8 {
            break;
        }
    }
}

pub fn loop_with_args_call() {
    // 50 - 61 arası degerleri enumerate yap
    // bu degerleri ekrana yazdır
    // 61 dahil değil

    for (poz, y) in (50..61).enumerate() {
        println!("{} : {}", poz, y);
    }
}
