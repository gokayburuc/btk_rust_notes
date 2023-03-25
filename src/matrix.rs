pub fn matris_call() {
    let mtx: [[f32; 3]; 2] = [[1.0, 2.0, 0.0], [12.0, 9.0, 5.0]];
    println!("{:?}", mtx);
    println!("elemanlar {:?} boyut:{}", mtx, mtx.len());
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("mtx[{}{}] = {}", i, j, mtx[i][j])
        }
    }
    println!("------index number eaqual--------");

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}
