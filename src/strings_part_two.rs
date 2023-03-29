pub fn string_call() {
    let isim = "Gokay";
    let selamlama = format!("merhaba {} iyi gunler dilerim", isim);

    print!("{}", selamlama);

    // repeat

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0} {1} {0}", run, forest);
    println!("{}", rfr);

    // info sample
    let info = format!(
        "{sehir} sehri {tarih} yılında feth edilmiştir.",
        sehir = "Edirne",
        tarih = "1356",
    );

    println!("{}", info);
}
