pub fn tuple_call() {
    let x: i32 = 3;
    let y: i32 = 4;
    let sonuc = topla_ve_carp(x, y);
    println!("sonuclar : {:?}", sonuc);

    // sonuc ifadesinde döndürülen degerler
    println!("{0} + {1} = {2} ve {0} * {1} = {3}", x, y, sonuc.0, sonuc.1);

    // return ifadesine degisken atanması
    let (toplam, carpim) = sonuc;
    print!("toplam: {} ve carpim: {}", toplam, carpim);

    // kombine
    let sonuc2 = topla_ve_carp(4, 8);
    let combine = (sonuc, sonuc2); // iki tuple iceren tuple
    print!("Sonuclar: {:?}", combine);

    // elemanlar
    let elemanlar = (true, 23.1, -118);
    println!("{:?}", elemanlar);

    // eleman
    let eleman = (17,);
    println!("{:?}", eleman);
}

fn topla_ve_carp(x: i32, y: i32) -> (i32, i32) {
    // toplam ve carpim tuple ifadesi - demet
    (x + y, x * y)
}
