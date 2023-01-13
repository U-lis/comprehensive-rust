fn main() {
    fizzbuzz_to(20);
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n)
    }
}

fn fizzbuzz(x: u32) {
    match (is_dividable_by(x, 3), is_dividable_by(x, 5)) {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{x}"),
    }
}

fn is_dividable_by(x: u32, r: u32) -> bool {
    if r == 0 {
        return false;
    }
    return x % r == 0;
}