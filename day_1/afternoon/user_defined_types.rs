struct Person {
    name: String,
    age: u8,
}

struct Point(i32, i32); // Tuple struct

// Enum
#[derive(Debug)] // Needed for print?
enum Direction {
    Left,
    Right,
}

#[derive(Debug)] // Needed for print?
enum PlayerMove {
    Pass, // Simple variant
    Move(Direction), // Tuple variant
    Teleport {x: u32, y:u32} // Struct variant
}

fn describe(p: &Person) {
    println!("{} is {} years old.", p.name, p.age);
}

// Const
const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE <10 {42} else {13}
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

// static
static BANNER: &str = "This is the static banner.";
static ANSWER:i32 = 42;
// Mutable static 도 있기는 한데 unsafe 임
// static mut REAL_ANSWER:i32 = 42;

fn main() {
    let p1 = Person{name: String::from("Alice"), age: 20};
    let name = String::from("Bob");
    let age = 22;
    let p2 = Person{ name, age };
    let p3 = Person{name: String::from("Charlie"), ..p2};
    describe(&p1);
    describe(&p2);
    describe(&p3);

    let pt = Point(13, 17);
    println!("{}, {}", pt.0, pt.1);

    // Enum
    let dir = Direction::Left;
    let player_move = PlayerMove::Move(dir);
    println!("{:?}", player_move);

    // Const
    let digest = compute_digest("Hello world!");
    println!("digest: {:?}", digest);

    // Static
    println!("{BANNER}");
    println!("{ANSWER}");
    println!("{}", ANSWER+1);
}