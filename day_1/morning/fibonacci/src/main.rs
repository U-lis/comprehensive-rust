fn fib(n: u32) -> u32 {
    if n <= 2 {
        // 기본 사례입니다.
        1
        // return 1;
    } else {
        // 재귀 사례입니다.
        fib(n-1) + fib(n-2)
        // return fib(n-1) + fib(n-2);
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
