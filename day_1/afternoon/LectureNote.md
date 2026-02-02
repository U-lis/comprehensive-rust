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
    let Foo {a, b} = foo; // Destructure struct.
    println!("a: {a}, b: {b}");
}

fn main() {
   let f = Foo {a:32, b:true};
   print_foo(f);
}
```
