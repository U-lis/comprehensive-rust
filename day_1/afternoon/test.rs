fn main() {
    // Array
    let mut a: [i8;10] = [42; 10];
    println!("{a:#?}");

    // Tuple
    let t : (i32, bool, &str) = (10, false, "This is Tuple");
    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);

    for e in a {
        println!("element {e}");
    }
    // To get index while iteration, use enumerate()
    for (index, value) in  a.iter().enumerate() {
        println!("{}: {}", index, value);
    }

    for i in 0..10 {
        a[i] = i as i8 * 2;
    }
    let [a, b, c, d, e, f, g, h, i, j] = a;
    println!("{a} {b} {c} {d} {e} {f} {g} {h} {i} {j}");

    // Destructure Struct
    let f = Foo {a:32, b:true};
    let f1 = f.a;
    let f2 = f.b;
    println!("f1: {f1}, f2: {f2}");

    print_foo(f);
}

struct Foo {
    a: i32,
    b: bool,
}

fn print_foo(foo: Foo) {
    let Foo {a, b} = foo;
    println!("a: {a}, b: {b}");
}