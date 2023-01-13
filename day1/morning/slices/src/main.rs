fn main() {
    let mut a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("a: {a:?}");
    let s: &[i32] = &a[2..4]; // [inclusive .. exclusive]
    println!("s: {s:?}");

    /*
    `let s` borrows a[2..4] from a to s.
    Thus a[3] cannot be modified.
    */
    // Uncommenting following line raises compile time error
    //a[3] = 100;
    //println!("s after modify a: {s:?}");
}
