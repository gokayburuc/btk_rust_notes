// collections isimli kütüphaneden hashmap  ve hashset i cagir
use std::collections::HashSet;

pub fn hashset_call() {
    // yeni bir hashmap oluştur

    let mut veri = HashSet::new();
    veri.insert("gama");
    veri.insert("delta");
    print!("{:?}", veri);

    let veri_eklendi_mi = veri.insert("vega");
    if veri_eklendi_mi {
        println!("Veri eklendi : vega");
    }

    // veri seti tek deger icerir

    veri.insert("delta");
    print!("{:?}", veri);

    // veri contains

    if !veri.contains("kappa") {
        println!("kappa yok ne yazık ki");
    }

    let eleman_sil = veri.remove("delta");

    if eleman_sil {
        println!("delta silindi");
    }

    println!("{:?}", veri);

    // 1 ve beş arası değerleri alan bir veri seti

    let _1_5: HashSet<i32> = (1..=5).collect();
    let _6_10: HashSet<i32> = (6..=10).collect();
    let _1_10: HashSet<i32> = (1..=10).collect();
    let _2_8: HashSet<i32> = (2..=8).collect();

    // is subset
    println!(
        "{:?} kümesi {:?} kümesinin alt kümesi midir? {}",
        _1_5,
        _1_10,
        _1_5.is_subset(&_1_10)
    );

    // is disjoint
    println!(
        "{:?} kümesi {:?} kümesi ile ayrık mı? {}",
        _2_8,
        _6_10,
        _2_8.is_disjoint(&_6_10)
    );

    // union
    println!(
        "{:?} kümesi {:?} ile birlisimi {:?}",
        _1_5,
        _2_8,
        _1_5.union(&_2_8)
    );
}
