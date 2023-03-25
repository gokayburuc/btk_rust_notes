union IntOrFloat {
    i: i32,
    f: f32,
}

pub fn sample_call() {
    let mut iof = IntOrFloat { i: 122 };
    iof.i = 221;

    let value = unsafe { iof.i };

    println!("value :{}", value);

    let iof2 = IntOrFloat { f: 55.0 };
    process_value(iof2);

    let iof3 = Intorfloat { i: 44 };
    process_value(iof3);

    let iof4 = IntOrFloat { i: 1 };
    process_value(iof4);
}

pub fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 44 } => {
                println!("degerimiz 44 tür")
            }
            IntOrFloat { f: f32 } => {
                println!("2. durum böyle ifade edilir deger {}", f);
            }
        }
    }
}
