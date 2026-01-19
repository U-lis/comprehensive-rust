# Lecture Note

## How to run rust

### Manual way

```bash
$ rustc xxx.rs  # to compile: create xxx executable
$ ./xxx  # Run executable
```

### Cargo

```bash
$ cargo new xxx  # This will create a new project named xxx
$ cd xxx
# Write your rust code
$ cargo run  # This will compile and run the code
```

## Types and Values

### variable

rust 에서 변수는 기본적으로 불변이다.  
let 으로 선언할 수 있고, type 을 원하면 `let x: i32 = 5;` 식으로 변수명 뒤에 : + type 을 붙인다.  
rust 의 타입은 컴파일 타임에 정해져야 하지만 타입 추론을 통해 많은 경우 생략할 수 있다.

```rust
fn main() {
    let x = 5;
    let y: i32 = 12;
    x = 10; // This will not compile: x is immutable
}
```

가변 변수를 선언하기 위해서는 mut 키워드를 사용해야 한다.

```rust
fn main() {
    let mut x = 5;
    let y: i32 = 12;  // This is still immutable
    x = 10;  // This will compile
}
```

### values

<table><thead><tr><th colspan="2">타입</th><th>리터럴 값</th></tr></thead><tbody>
<tr><td>부호있는 정수</td><td><code class="hljs">i8</code>, <code class="hljs">i16</code>, <code class="hljs">i32</code>, <code class="hljs">i64</code>, <code class="hljs">i128</code>, <code class="hljs">isize</code></td><td><code class="hljs">-10</code>, <code class="hljs">0</code>, <code class="hljs">1_000</code>, <code class="hljs">123_i64</code></td></tr>
<tr><td>부호없는 정수</td><td><code class="hljs">u8</code>, <code class="hljs">u16</code>, <code class="hljs">u32</code>, <code class="hljs">u64</code>, <code class="hljs">u128</code>, <code class="hljs">usize</code></td><td><code class="hljs">0</code>, <code class="hljs">123</code>, <code class="hljs">10_u16</code></td></tr>
<tr><td>부동소수</td><td><code class="hljs">f32</code>, <code class="hljs">f64</code></td><td><code class="hljs">3.14</code>, <code class="hljs">-10.0e20</code>, <code class="hljs">2_f32</code></td></tr>
<tr><td>유니코드 문자</td><td><code class="hljs">char</code></td><td><code class="hljs">'a'</code>, <code class="hljs">'α'</code>, <code class="hljs">'∞'</code></td></tr>
<tr><td>불리언</td><td><code class="hljs">bool</code></td><td><code class="hljs">true</code>, <code class="hljs">false</code></td></tr>
</tbody></table>

`isize`, `usize` 는 pointer의 크기와 동일하다.  
char은 32bit, bool은 8bit.  
긴 숫자는 `1_000` 식으로 표현할 수 있다. `_`는 전부 다 무시된다.

### Calculations

산술 연산은 일반적인 다른 언어와 동일하게 처리된다.

### String

rust에는 두 가지 문자열 타입이 있다. 둘 다 항상 UTF-8 로 인코딩된다.

- `String`: 변경 가능하고, 사용자가 소유하고 있는 문자열.
- `&str`: 읽기 전용. 문자열 literal 은 전부 이 타입이다.

rust에서의 문자열 slicing 은 우리가 생각하는 글자 개수 slicing이 아닌 byte 기준 slicing 이다.  
따라서 `&str[0..k]` 은 앞에서부터 k개의 글자가 아닌 k byte 를 출력하라는 것이고, 경우에 따라 글자 중간에서 index가 멈추게 되면 에러가 발생할 수 있다.

```rust
fn main() {
    let greeting: &str = "Hello";  // This is a string literal
    let mut name = String::new();  // This is a mutable string
    name.push_str("World");
    println!("{} {}!", greeting, name);  // Hello World!
    println!("{:?}", &greeting[0..3]);  // string slice in rust is based on bytes, not characters
    prinln!("{:?}", &name[0..2]);  // mutable String can be treated as immutable str
} 
```

### Type inference

rust 에서는 코드 내에서 변수가 어떻게 선언되어 어떻게 사용되는지를 보고 타입을 추론한다.  
정수 타입의 기본 추론 타입은 `i32`이며, 부동소수점의 경우 `f64`이다.

```rust
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x = 10;  // used by takes_u32, inferred to be u32
    let y = 20;  // used by takes_i8, inferred to be i8

    takes_u32(x);
    takes_i8(y);
    takes_u32(y); // This will not compile: y is inferred to be i8
}
```
