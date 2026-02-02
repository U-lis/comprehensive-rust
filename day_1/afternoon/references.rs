fn main () {
    // Shared Reference
    let a = 'a';
    let mut b = 'b';
    let mut rf: &char = &a;
    dbg!(rf); // Shows reference info
    println!("{}", rf); // println 에서 알아서 펴주는건가...? a 가 나온다.
    println!("{}", *rf);

    rf = &b;
    dbg!(rf);

    // This will not work: `rf` is a `&` reference, so the data it refers to cannot be written
    //*rf = 'c';

    // Exclusive Reference
    let mut c = 'c';
    let mut rf2 = &mut c;
    *rf2 = 'd';
    // This will not work: cannot borrow `c` as immutable because it is also borrowed as mutable
    // let rf3 = & c;

    // This will not work: cannot borrow `c` as mutable more than once at a time
    // let rf3 = &mut c;

    println!("{}", c);

    let mut x = 'x';
    rf2 = &mut x;
    *rf2 = 'y';
    println!("{}", x);

    // Slice
    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("arr1: {:?}", arr1);
    let mut s: &[i32] = &arr1[2..5];
    println!("s: {:?}", s);
    // This will not work
    // s[1] = 100;

    let mut arr2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("arr2: {:?}", arr2);
    let ss = &mut arr2[2..5];
    println!("ss: {:?}", ss);
    ss[1] = 100;
    println!("arr2 after change: {:?}", arr2);

    // Can reuse slice with a different length
    s = &arr1[..4];
    println!("slice_first: {:?}", s);


    // String
    let s1: &str = "World";
    let mut s2: String = String::from("Hello");
    s2.push_str(s1);
    let s3: &str = &s2[2..8];
    println!("{s3}");
    // Byte string
    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]); // "abc"
    // Raw string
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}