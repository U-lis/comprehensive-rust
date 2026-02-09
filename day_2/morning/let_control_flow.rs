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

    let mut name = String::from("Comprehensive Rust");
    while let Some(c) = name.pop() { // pop() returns Some(char) until string is empty. And then returns None.
        dbg!(c);
    }

}