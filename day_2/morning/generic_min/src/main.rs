use std::cmp::Ordering;

// TODO: implement the `min` function used in the tests.
fn min<T: Ord>(this: T, that:T) -> T {
    match this.cmp(&that) {
        Ordering::Less => this,
        Ordering::Equal => this,
        Ordering::Greater => that
    }
}


#[test]
fn integers() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
}

#[test]
fn chars() {
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
}

#[test]
fn strings() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
