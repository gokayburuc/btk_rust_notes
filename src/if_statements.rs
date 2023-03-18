pub fn sicaklik() {
    let sicaklik: i32 = 22;

    if sicaklik > 35 {
        println!("cok sicak");
    } else if sicaklik < 10 {
        println!("Cok soguk");
    } else {
        println!("İdeal!")
    }

    // burada if else i degisken icinde tanımladık
    let gun = if sicaklik > 20 { "Gunesli" } else { "bulutlu" };
    println!("gun : {}", gun);

    // hava durumu isimli baska bir degisken tanımlıyoruz
    println!(
        "Hava Durumu {}",
        if sicaklik > 20 {
            "Sicak"
        } else if sicaklik < 10 {
            "Soguk"
        } else {
            "İdeal"
        }
    );

    // nested if

    println!(
        "Hava Durumu Ozeti : {}",
        if sicaklik > 20 {
            if sicaklik > 30 {
                "cok sicak"
            } else {
                "sicak"
            }
        } else if sicaklik < 10 {
            "soguk"
        } else {
            "ideal"
        }
    );
}
