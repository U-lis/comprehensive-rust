# Lecture Note

## Array

```rust
fn main() {
    // 10 elements of i8 type array. The size of the array is fixed.
    // Different length array are different type.
    let mut a: [i8; 10] = [42; 10]; // 10 of 42 are inside the initial array.
    a[5] = 10;
    println!("{:?}", a); // To print array, use {:?} format because it's a array.
    println!("{a:?}"); // Inline variable
    println!("{a:#?}", a); // Prettier print
}
```

Uninitialized array can be defined but is not usable before initialization.  
Accessing to the wrong index is checked at runtime.

## Tuple

```rust
fn main() {
    // Tuple can combine multiple individual types into one tuple type.
    let t: (i8, bool) = (7, true); // Tuple of i8 and bool. this does not mean type and length.
    // Can access a tuple element using dot notation.
    println!("t.0: {}", t.0);
}
```

`()` is empty tuple. This signifies the absence of a return value. (void?)

## Iteration

`for` loop can iterate over array, but not tuple.

```rust
fn main() {
    let arr: [i8; 5] = [19; 5];
    let tup: (i8, bool) = (19, true);

    for e in arr {
        println!("element: {e}");
    }
    // This does not work
    for e in tup {
        println!("element: {e}");
    }
}
```

You can use `enumerate()` function to get index of each element.

```rust
fn main() {
    let arr: [i8; 10] = [10; 10];
    for (i, e) in arr.iter().enumerate() {
        println!("{i}th element: {e}");
    }
}
```

## Destructuring

You can destruct tuple and array by accessing its element directly.

```rust
fn main() {
    let tup: (i8, bool) = (10, false);
    let first = tup.0; // 10
    let second = tup.1; // false

    let arr: [i8; 3] = [1, 2, 3];
    let a1 = arr[0]; // 1
    let a2 = arr[1]; // 2
    let a3 = arr[2]; // 3
}
```

But you can use pattern matching to destruct tuple.

```rust
fn main() {
    let tup: (i8, bool) = (10, false);
    let (first, second) = tup; // Type and length of tuple is inferred.
}
```

You can destruct array in the same way.

```rust
fn main() {
    let mut arr: [i8; 10];
    for i in 0..10 {
        arr[i] = i as i8 * 2;
    }
    let [a1, a2, a3, a4, a5, a6, a7, a8, a9, a10] = arr; // Destructure array.
    println!("{a1}, {a2}, {a3}, {a4}, {a5}, {a6}, {a7}, {a8}, {a9}, {a10}");
}
```

You can destruct struct too.

```rust
struct Foo {
    a: i32,
    b: bool,
}

fn print_foo(foo: Foo) {
    let Foo { a, b } = foo; // Destructure struct.
    println!("a: {a}, b: {b}");
}

fn main() {
    let f = Foo { a: 32, b: true };
    print_foo(f);
}
```

## Reference

### Shared Reference

`T` Type 변수의 Shared reference 는 `&T` type 을 가진다.  
`*` 를 사용해서 reference 안의 값을 꺼낼 수 있다.
// 어 이거 완전 포인터...  
reference 는 null 일 수 없다. 그래서 별도의 null check 가 필요 없다.

```rust
fn main() {
    let a = 'a';
    let b = 'b';

    let mut r: &char = &a;
    println!("{r}");
    r = &b; // rebind
    println!("{r}");

    // This will not work
    *r = 'c';
}
```

Ownership 과 헷갈리면 안 됨: reference 는 pointer 처럼 값을 빌려와서 쓸 수 있게 하는 것.  
여전히 이 값의 소유권은 원래 변수에 있다. ownership 은 나중에 공부함.

함수를 호출할 때를 제외하고는 `&` 를 이용해 명시적으로 referencing 하는 것이 필요하다.  
함수를 호출할 때는 알아서 referencing, dereferencing 이 작동한다.
그래서 C++ 에서 하는 것처럼 `->` 연산자가 있을 필요가 없다.

`r = &b;` 구문을 보면 r 이 mutable 이기 때문에 &r 로 재할당되어서 다른 것을 가리킨다. C++ 등에서 참조된 값을 변경하는 것과 대조적이다. // 이쪽이 더 직관적인 느낌이긴 하네  
`&` 를 이용한 shared reference 는 원본 변수가 mutable 이라 해도 값의 변경을 허락하지 않는다. 예를 들어서 `*r = 'c'` 는 작동하지 않는다.

rust 에서는 reference 들의 lifetime을 추적한다. dangling reference 라는 것은 safe rust 에서는 발생할 수 없다.  
관련 이야기는 ownership 에서 더 다룰 예정.

### Exclusive Reference

Mutable reference 라고도 한다. 독점적 참조.  
이게 한번 만들어지면 이 변수에 대한 다른 reference 는 shared, exclusive 에 상관 없이 더 생성될 수 없다. 이 부분이 포인터와 다른 점.  
이건 shared reference 와 다르게 원본 값의 변경이 가능하다.  
`T` type의 변수에 대한 exclusive reference 는 `&mut T` type 을 가진다.

```rust
fn main() {
    // let a = 'a'; 에 대해서는 r 이 동작하지 않는다.
    let mut a = 'a';
    let r = &mut a;
    *r = 'b';
    println!("{a}");
}
```

### Slice

큰 array 같은 데이터에서 일부만 빌려와 view 를 생성할 수 있다.  
slice 는 원본에서 데이터를 빌려 온다. shared reference 처럼 동작.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let s: &[i32] = &a[2..5];
    println!("{s:?}");
}
```

array 앞쪽을 전부 가져오려면 `&a[..5]` 식으로 앞을 생략하면 된다.  
array 뒤쪽을 전부 가져오려면 `&a[2..]` 식으로 뒤를 생략하면 된다.  
array 전체를 reference 하려면 `&a[..]` 으로 간단히 할 수 있다.

#### NOTE

- s 의 type 을 보면 array 선언과 달리 길이에 대한 이야기가 없다. slice 는 여러 길이로 쓸 수 있다는 것.
- 대신 한번 만든 slice 의 길이를 늘릴 수는 없다. slice 는 buffer 를 더 가지고 있는 형태가 아니기 때문에 append 가 불가능하다.
- 더 큰 slice 을 만들고 싶으면 다시 원본에서 slice 를 잘라와야 한다.

### String

이제 두 종류의 string 을 이해할 수 있다.
하나는 `&str` 인데 이건 흡사 C 에서 문자열을 `char[]` 로 해 포인터를 사용하는 것과 유사하다. 문자열 literal 의 shared reference 로 볼 수 있다.  
또 다른 하나, `String` 은 `Vec<T>` 같이 메모리 버퍼 공간에 따로 잡는 느낌.

```rust
fn main() {
    let s1: &str = "World";
    let mut s2: String = String::from("Hello");
    s2.push_str(s1);
    let s3: &str = &s2[3..7];
    println!("{s3}");
}  
```

이전에 삽질하면서 알았듯 String slice 는 byte 를 기준으로 하고 있기 때문에 글자 boundary 에 맞지 않는 slice 는 에러를 낼 수 있다.  
byte string 을 사용하면 `&[u8]` 을 바로 만들 수 있다.

```rust
fn main() {
    println!("{:?}", b"abc"); // 97, 98, 99
    println!("{:?}", &[97, 98, 99]);
}
```

Raw string (`r#"~~~"#`) 을 사용해 escape 를 할 수 있다. `\`를 이용해 escape 할 수도 있다.

```rust
fn main() {
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}
```

### Reference Validity

Rust 에서는 reference 의 안전한 사용을 위한 몇가지 규칙을 가지고 있다. 그 중 하나는 reference 는 null 이 될 수 없다는 것. null pointer 가 있는 것과 대조적.  
그래서 rust 에서는 null check 없이 reference 를 사용할 수 있다.

```rust
fn main() {
    let ref_x = {
        let x = 5;
        &x
    };
    // This will not work
    dbg!(ref_x);
}
```

위 코드를 보면 ref_x 는 x 의 reference 여야 한다. 그러나 x 의 scope 는 `{}` 안쪽에만 존재하기 때문에 결과적으로 ref_x 는 유효한 reference 를 생성하지 못한다.  
이게 단순 pointer 였다면 x 의 scope 에 상관 없이 메모리 주소는 ref_x 에 남아 접근할 수 있었을 것이다. 이는 중대한 차이.

## User-defined Type

### Named Struct

C, C++ 처럼 custom struct 를 만들 수 있다.  
C 와 달리 typedef 는 필요없고 struct 만 있으면 된다.  
C++와 달리 struct 간의 상속은 없다.  
struct field 는 default value 를 지원하지 않는다. trait 에 관련된 게 있는데 나중에.

```rust
struct Person {
    name: String,
    age: u8,
} // no semicolon

// & 를 이용해 불변 reference 로 안전한 사용
fn describe(p: &Person) {
    println!("{p.name} is {p.age} years old.");
}

fn main() {
    let mut p = Person {
        name: String::from("Alice"),
        age: 20,
    };
    describe(&p);

    p.age = 21;
    describe(&p);

    let name = String::from("Bob");
    let age = 22;
    let p2 = Person { name, age }; // 변수명이 struct field 와 같은 이름인 경우 바로 사용 가능.
    describe(&p2);
}
```
이미 있는 struct instance 에서 값을 가져오되 explicit 만 덮어쓸 수도 있다.

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p1 = Person{name: String::from("Alice"), age: 20};
    let p2 = Person{ name: String::from("Bob"), ..p1}; // Use age from p1
}
```

### Tuple Struct
field name 이 중요하지 않은 경우 tuple struct 를 사용할 수 있다.
```rust
struct Point(i32, i32);

fn main() {
    let p = Point(13, 17);
    println!("{p.0}, {p.1}");
}
```

single field tuple structs 는 종종 type wrapper 처럼 사용된다.
```rust
struct Wrapper(i32);

fn calc(inp: Wrapper) -> Wrapper {
    todo!("do something")
}

fn main() {
    let w = Wrapper(13);
    println!("{w.0}");
}
```

### Enum
enum 을 사용하면 정해진 variation 을 가진 값들을 모아 사용할 수 있다.  
enum value 에 접근할 때는 `::` 을 사용한다.  

```rust
enum Direction {
    Left,
    Right,
}

// 여러가지가 enum 에 들어갈 수 있다. 알아서 어떤건지 찾아준다.
enum PlayerMove {
    Pass, // Simple variant
    Move(Direction), // Tuple variant
    Teleport {x: u32, y:u32} // Struct variant
}

fn main() {
    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Move(dir);
    println!("go to {player_move:?}");
}
```

### Type Alias
alias 를 이용해 다른 타입에 대한 이름을 지을 수 있다. 양쪽 이름은 서로를 오갈 수 있다.  
C 의 typedef 와 유사하다고 볼 수 있다.
```rust
enum Direction {
    Left,
    Right,
}
type Dir = Direction;

// 복잡한 타입에 유용함.
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;
```

### Const
const compile time 에 평가되어서 code 에 inline 된다. 당연히 수정 안된다.
```rust
const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fil_value(); // const function 은 compile time 에 불려서 평가된다.

// const fn 은 runtime 에 부를 수 있다.
const fn calculate_fil_value() -> u8 {
    if DIGEST_SIZE < 10 {42} else {13}
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}
```

### Static
static 은 program 내내 살아서 유지된다. 그래서 변경도 없다.  
```rust
static BANNER: &str = "Hello, World";
fn main() {
    println!("{BANNER}");
}
```
