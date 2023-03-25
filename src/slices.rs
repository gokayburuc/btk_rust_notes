fn get_slice(slice: &mut [i32]) {
    print!(
        "Veri diliminin ilk elemanı {} eleman sayisi {}",
        slice[0],
        slice.len()
    );

    slice[0] = 3000;
}

pub fn slice_call() {
    // mutable array
    let mut data = [1, 2, 3, 4, 5];
    get_slice(&mut data[1..4]);

    println!("{:?}", data);
}
