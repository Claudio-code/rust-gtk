fn main() {
    // move ownership to another variable
    let x = vec!["test"];
    println!("value {:?}", x);
    let y = x;
    // it line throw error
    // println!("value {:?}", x);
    println!("value {:?}", y);

    // clone
    let cloned = y.clone();
    println!("value {:?}", cloned);
}
