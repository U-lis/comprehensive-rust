fn main() {
    let mut x: i32 = 10;
    println!("x: {x}");
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    println!("=====");
    danging();
}

fn danging() {
    let ref_x : &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
        println!("x ref_x inside: {ref_x}");
    }
    // Uncommenting this raises compile time error
    // println!("x ref_x outside: {ref_x}");
}
