use std::collections::HashMap;

pub fn hashmap_call() {
    let mut sekiller = HashMap::new();

    let ucgen = String::from("ucgen");

    sekiller.insert(ucgen, 3);

    sekiller.insert("kare".into(), 4);

    println!("bir karenin {} kenari vardır", sekiller["kare"]);

    for (k, v) in &sekiller {
        println!("{}:{}", k, v);
    }

    println!("---------------------------");
    sekiller.insert("kare".into(), 66);
    println!("{:?}", sekiller);

    //varsa al yoksa ekle
    sekiller.entry("daire".into()).or_insert(360);

    println!("{:?}", sekiller);

    sekiller.entry("kare".into()).or_insert(2500);

    {
        let suan = sekiller.entry("yamuk".into()).or_insert(1234);
        *suan = 0;
    }

    println!("{:?}", sekiller);
}
