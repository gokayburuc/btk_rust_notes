use ::std::mem;

pub fn array_call() {
    // 5 tane deger iceren ve degerler i32 olan array
    // let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    // create a mutable array
    let mut a = [1, 2, 3, 4, 5];

    println!(
        "a dizisi {} elemana sahiptir ve ilk elemanı : {}",
        a.len(),
        a[0]
    );

    a[0] = 22;

    println!("{}", a[0]);

    // TEST ASSERT
    // assert_eq!(a, [1, 2, 3, 4, 5]);

    // let b = [1; 10];

    // bellek yönetimi eleman boyutu
    let b = [5u16; 20];

    for i in 0..b.len() {
        // println!("{}", i);

        println!("{}. eleman {}", i + 1, b[i]);
    }

    println!("------------------------------");
    println!("{:?}", b);

    println!("------------------------------");
    println!("b dizisi {} byte boyutundadır.", mem::size_of_val(&b));
}
