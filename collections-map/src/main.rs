use std::collections::HashMap;

fn main() {
    // works like hashmap in java
    let mut map = HashMap::<i32, i32>::new();

    map.insert(1, 22);
    map.insert(22, 2);

    map.remove(&21);

    println!("{:?}", map);
}
