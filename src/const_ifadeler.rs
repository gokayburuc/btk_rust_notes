const SABIT_SAYI: i32 = 44; // const
static m: i32 = 20; // static

static mut z: i32 = 33;

// static degerler mutable olabilir
// unsafe ile duzenlenir

fn main() {
    println!("{}", SABIT_SAYI);

    unsafe {
        z = 45;
        println!("{}", z);
    }
}
