use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let s = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", s);

    let d: Point = serde_json::from_str(&s).unwrap();
    println!("deserialized = {:?}", d);
}
