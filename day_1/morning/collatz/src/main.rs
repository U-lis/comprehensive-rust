/*
콜라츠 수열 : 양의 정수 N이 주어졌을 때, 이 값이 짝수이면 2로 나누고, 홀수이면 3을 곱하고 1을 더한다. 계산 결과값이 1이 되면 수열이 종료된다.
*/
/// `n`에서 시작하는 콜라츠 수열의 길이를 결정합니다.
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

fn collatz_length_recursive(n: i32) -> u32 {
    if n == 1 {
        return 1;
    }

    if n % 2 == 0 {
        1 + collatz_length_recursive(n / 2)
    } else {
        1 + collatz_length_recursive(3 * n + 1)
    }
}

fn main() {
    let n = 10239;
    println!("collatz_length({}) = {}", n, collatz_length(n));
    println!("collatz_length_recursive({}) = {}", n, collatz_length_recursive(n));
}
