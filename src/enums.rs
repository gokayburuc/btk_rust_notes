// yapılara sablon olusturan bir kalıp olarak dusunulebilir

// struct Point {
//     x: f64,
//     y: f64,
// }

// bir struct ifadesi bircok alt struct ifadesinden olusabilir

// struct Line {
//     start: Point,
//     end: Point,
// }

// enum ifadeleri

enum Color {
    Mavi,
    Kirmizi,
    Yesil,
    RGBColor(u8, u8, u8), // tuple
    CMYKColor {
        yellow: u8,
        magenta: u8,
        cyan: u8,
        black: u8,
    },
}

pub fn enums() {
    let _p = Color::Mavi;
    let _b = Color::Yesil;
    let _t = Color::Kirmizi;
    let _c = Color::RGBColor(0, 0, 0);
    let _h = Color::RGBColor(1, 2, 0);
    let c = Color::CMYKColor {
        yellow: 255,
        magenta: 255,
        cyan: 255,
        black: 255,
    };

    match c {
        Color::Mavi => println!("mavi renk"),
        Color::Kirmizi => println!("kirmizi renk"),
        Color::Yesil => println!("yesil renk"),
        Color::RGBColor(0, 0, 0) => println!("siyah renk"),
        Color::CMYKColor {
            yellow: 0,
            magenta: 0,
            cyan: 0,
            black: 0,
        } => println!("siyah renk"),
        _ => {
            println!("liste dışı renk")
        }
    }
}
