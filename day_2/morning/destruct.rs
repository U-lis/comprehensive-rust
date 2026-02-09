enum Result {
    Ok(i32),
    Remain1(String),
    Err(String),
}

fn divide_by_three(inp: i32) -> Result {
    if inp % 3 == 0 {
        Result::Ok(inp/3)
    }
    else if inp % 3 == 1 {
        Result::Remain1(format!("{inp} remains only 1"))
    }
    else {
        Result::Err(format!("{inp} has too many remains"))
    }
}

fn main() {
    let inp:i32 = 10;
    match divide_by_three(inp) {
        Result::Ok(n) => println!("{inp} divided by 3 is {n}"),
        Result::Remain1(msg) => println!("{msg}"),
        _ => println!("Error"),
    }
}