fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 2;
    let y: i16 = 8;

    // Rust does not support implicit type casting
    // println!("x * y = {}", multiply(x, y));
    println!("x * y = {}", multiply(x.into(), y));
}

