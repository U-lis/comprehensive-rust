fn main() {
    // array : must have same type, fixed length
    // [T; N] or [10, 20, 30
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}"); // {:?} == Debug output

    // Tuple : fixed length
    // (), (T1,), (T1, T2), ... or (), ('a'), ('a', true), ...
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);  // Access using {dot + index}
    println!("2nd index: {}", t.1);
}
