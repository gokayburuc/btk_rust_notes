pub fn match_case_call() {
    let ulke_kodu = 46;

    let ulke = match ulke_kodu {
        44 => "UK - United Kingdom",
        46 => "SE - Sweeden",
        1 => "US - United States",
        7 => "RU - Russia Federation",
        90 => "TR - Republic Of Turkey",
        1..=1000 => "Other Country",
        _ => "Invalid Number",
    };

    println!("{}  : {} ", ulke_kodu, ulke);

    // boolean match

    let islem_sonucu = false;

    let sonuc = match islem_sonucu {
        false => "HATALI",
        true => "UYGUN",
        // _ => "ONGORULEMEYEN SONUC",
    };

    println!("Dondurulen İfade: {} , Sonuc : {} ", islem_sonucu, sonuc);
}
