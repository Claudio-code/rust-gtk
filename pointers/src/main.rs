fn main() {
    box_deferency();
}

fn box_deferency() {
    let tuple = (12, "apps"); // create on the stack
    let tuple_box = Box::new(tuple); // create referece pointer on the heap, but was stored on the stack
    println!("{:?}", tuple_box);

    let x = 5; // has value on the stack
    let y = &x; // has referece of value on the stack
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:?}", y);
}
