use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("thread spawn = {}", i);
        }
    });

    for i in 1..5 {
        println!("main = {}", i);
        thread::sleep(Duration::from_millis(100));
    }
}
