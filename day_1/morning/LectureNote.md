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
let x = 5;
let y: i32 = 12;
x = 10; // This will not compile: x is immutable
```

가변 변수를 선언하기 위해서는 mut 키워드를 사용해야 한다.

```rust
let mut x = 5;
let y: i32 = 12;  // This is still immutable
x = 10;  // This will compile
```
