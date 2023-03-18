pub fn while_loop() {
    let mut x: i32 = 1;

    while x < 1000 {
        x *= 2; // x i 2 ile carp ve x olarak ata

        if x == 128 {
            continue;
        }

        println!("{}", x);
    }
}
