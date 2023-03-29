pub fn vector_call() {
    // let mut a = Vec::new();

    // // deger ekleme
    // a.push(1);
    // a.push(2);
    // a.push(3);

    // println!("{:?}", a);
    // a.push(4);
    // println!("{:?}", a);

    let mut a: Vec<i32> = vec![1, 2, 3, 4]; // [1;10]
    println!("{:?}", a);

    //
    let index: usize = 9;
    // println!("a[0] ={}", a[index]);

    println!("a[9] = {}", a[index]); // panic statement

    match a.get(11) {
        Some(x) => println!("a[11] = {}", x),
        None => println!("deger yok!"),
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(55);
    println!("{:?}", a);

    let son_eleman = a.pop();
    println!("son eleman : {:?} ve a vektörü:{:?}", son_eleman, a);
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
