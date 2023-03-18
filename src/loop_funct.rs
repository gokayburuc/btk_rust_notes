pub fn loop_call() {
    let mut x = 1;
    print!("Loop Calisiyor!\n");
    loop {
        x *= 2;
        print!("{}\n", x);
        if x == 1 << 10 {
            print!("Loop 10 Defa Calisti ve Durdu");
            break;
        }
    }
}
