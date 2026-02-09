# Pattern Matching

## Irrefutable Patterns

```text
irrefutable: 반박할 수 없는, 부인할 수 없는
```

tuple분해할 때 이미 pattern matching 을 사용하고 있었다.  
개수가 맞으면 type 은 알아서 추론한다.  
필요하다면 일부 인자를 변수에 할당하지 않고 무시할 수도 있다.

```rust
fn main() {
    let tup: (i32, bool, f32) = (42, false, 3.1415927);
    let (a, b, c) = tup; // a:i32, b:bool, c:f32
    let (_, y, z) = tup; // Ignore the first element of the tuple
    let (xx, _, zz) = tup; // Ignore the second element of the tuple
    let (.., last) = tup; // Ignore all elements except the last one

    let big_tup: (bool, char, i8, f32) = (true, 'c', -1, 3.1415927);
    let (f, .., l) = big_tup; // .. can ignore the middle elements at once.
}
```

## Matching Values

`match` keyword 를 사용하면 간단하게는 C 의 switch 구문과 같이 사용할 수 있다.  
하지만 rust 의 `match` 는 이 이상의 복잡한 패턴 매칭도 가능하다.

```rust
fn main() {
    let input = 'x';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Something else"),
    }
}
```

arm 중에 `key` 변수가 사용되고 있는데, 이건 이전에 선언된 적이 없다. 이런 변수는 arm 내부용 변수로 자동적으로 binding 된다.  
C의 switch와 달리 rust 의 match arm 에는 break 가 없다. `=>` 하나에 하나씩만 수행된다.  
한번에 여러 작업을 arm내에서 하고 싶으면 {} 으로 블록을 만들면 된다. 블록의 마지막 expression 결과가 arm의 반환값이 된다.  
match arm 안에 있는 if 는 match 가 완료된 뒤에 평가된다. 그래서 if 에서 실패했더라도 match 를 다시 찾지 않는다.  
match guard 안에서 condition check 를 하려면 `|` 등을 이용해 처리해야 한다.  
match arm 의 조건에 이미 존재하는 변수를 넣어서 비교할 수 없다.

```rust
fn main() {
    let expected = 5;
    match 123 {
        expected => println!("Expected value is 5, actual is {expected}"),
        _ => println!("Value was something else"),
    }
}
```

위 예시에서 expected가 5로 되어 있지만 match 안의 expected 는 5가 아닌 내부 변수로 선언되며, 항상 선택된다.  
따라서 위 예시를 수행하면 항상 "Expected value is 5," 어쩌구 하는 구문이 선택된다.

`@ syntax` 라는걸 사용해 패턴의 일부를 변수로 사용할 수 있다. 잘 쓰지는 않지만 유용하다.

```rust
fn main() {
    let opt = Some(123);
    match opt {
        outer @ Some(inner) => {
            println!("outer: {outer:?}, inner: {inner}");
        }
        None => {}
    }
}
```

## Destructing Structs

tuple 처럼 구조체도 분해할 수 있다.

```rust
struct Move {
    delta: (i32, i32),
    repeat: u32,
}

#[rustfmt::skip]
fn main() {
    let m = Move { delta: (10, 0), repeat: 5 };

    match m {
        Move { delta: (0, 0), .. }        => println!("Standing still"),
        Move { delta: (x, 0), repeat }    => println!("{repeat} step x: {x}"),
        Move { delta: (0, y), repeat: 1 } => println!("Single step y: {y}"),
        _                                 => println!("Other move"),
    }
}
```

첫번째는 `delta: (0, 0)` 이기만 하면 들어간다.  
두번째는 delta 의 y 가 0이고, repeat는 들어온대로 쓰인다.  
세번째는 delta 의 x 가 0이고, repeat 가 1일 때만 작동한다.  
delta 는 내부에서 한번 더 체크하는, nested pattern 임을 유의할 것.

위 예시에서 `match &m` 또는 `match &mut m` 을 이용해 reference 를 match에 사용할 수 있다.  
pattern match자체는 m 일 때와 동일하지만, match arm 내에서 reference를 사용하게 된다.

capture와 const expression 이 구분하기 좀 어렵다.

## Destruct Enums

tuple처럼 enum 도 분해 가능하다.

```rust
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}
```

위 예시에서 Result 내의 값들을 사용했는데, half, msg가 destruct되어서 자동으로 매칭된다.  
Enum 을 destruct 할 때에는 모든 케이스를 다 커버해야 한다. 최소한 `_` 라도 있어서 모든 케이스를 처리해야 한다.  
enum 안에 있는 값은 match 가 완료되어 분기를 고른 뒤에만 접근할 수 있다. condition 검증할 때 안에 있는 값을 미리 체크할 수 없다.

## Let control flow

rust 에는 let + control flow 를 동시에 할 수 있는 방법이 있다.  
`if let`, `while let`, `let else` 가 그것이다.

### `if let`

pattern match 를 let 안에 넣어서 이에 따라 다른 condition 을 작동하게 할 수 있다.

```rust
use std::time::Duration;

fn sleep_for(secs: f32) {
    let result = Duration::try_from_secs_f32(secs);

    if let Ok(duration) = result {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    } else {
        println!("cannot sleep for {secs} seconds");
    }
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);
}
```

match와 달리, if let 은 모든 branch 를 다 커버하지 않는다. 위 예시에서는 try_from_secs_f32 가 Ok인 경우만 작동한다.  
당연히 else 를 붙일 수 있다.

### `while let`

while let 은 통상적으로 어떤 값 안의 요소를 길이에 상관없아 전부 다 순회하고자 할 때 쓸 수 있다.

```rust
fn main() {
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        dbg!(c);
    }
}
```

pop() 이 string 을 돌면서 `Some(char)` 를 계속 반환한다.  
그러다 string이 비면 None 을 반환하고, 그때 `let Some(c)` 가 `False` 가 되기 때문에 while 문을 나오게 됨.  
사실 이건 무한루프에 if-else 를 사용해도 만들 수 있다. 일종의 문법 sugar 로 생각하면 됨.  
while let은 값이 없을 수 있기 때문에 expression 으로 사용할 수 없다.

### `let else`

pattern matching 도중 함수에서 return하는 것이 필요할 경우 let else를 사용할 수 있다.  
`else` case는 diverge 해야 한다. (return, break, panic등. 뭐가 됐건 block 끝에서 걍 정리되면 안 된다.)

```rust
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("got None"));
    };

    let first_byte_char = if let Some(first) = s.chars().next() {
        first
    } else {
        return Err(String::from("got empty string"));
    };

    let digit = if let Some(digit) = first_byte_char.to_digit(16) {
        digit
    } else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}

fn hex_or_die_trying_re(maybe_string: Option<String>) -> Result<u32, String> {
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };

    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}

fn main() {
    println!("result: {:?}", hex_or_die_trying(Some(String::from("AXa"))));
}
```
rust 에서는 이런 식으로 let else 를 이용해 early return 하는 식의 flow 를 많이 사용한다.  
