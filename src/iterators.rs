pub fn iters() {
    let mut vec = vec![2, 34, 35];

    for x in &vec {
        println!("deger :{}", *x);
    }

    for x in vec.iter() {
        println!("iteratif : {}", x);
    }

    for x in vec.iter().rev() {
        println!("tersten okuma {}", x);
    }

    for x in vec.iter_mut() {
        // her elemana 4 ekle
        *x += 4;
    }

    let mut vec2 = vec![2, 6, 3];

    vec2.extend(vec);
    println!("{:?}", vec2);
}
