pub fn advanced_func_call() {
    let limit = 500;
    let mut sum = 0;

    // if let limit_kadar= |y: u32| y < limit;
    //
    let limit_kadar = buyuk_mu(limit);

    for i in 0.. {
        let deger = i * i;

        if limit_kadar(deger) {
            break;
        } else if is_even(deger) {
            sum += deger;
        }

        println!("sum = {}", sum);
    }

    let toplam = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);

    println!("fonksiyonlar ileri yöntemle toplam: {}", toplam);
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn buyuk_mu(limit: u32) -> impl Fn(u32) -> bool {
    move |y: u32| y > limit
}
